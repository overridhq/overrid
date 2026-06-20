import type {
  AdminCapabilitiesResponse,
  AdminCapabilityFlag,
  AdminErrorResponse,
  AdminSessionContext,
  IdempotencyKey,
  OverridRef,
  ReasonCode,
  TraceId,
} from "./contracts";
import type { AdminRequestOptions } from "./overgate_client";

export type SessionContextStatus = "uninitialized" | "loading" | "loaded" | "failed" | "stale" | "permission_denied";

export interface SessionContextState {
  readonly status: SessionContextStatus;
  readonly context?: AdminSessionContext;
  readonly capabilities?: AdminCapabilitiesResponse;
  readonly selectedEnvironment?: AdminSessionContext["selected_environment"];
  readonly activeTenantId?: OverridRef;
  readonly actorId?: OverridRef;
  readonly fetchedAtEpochMs?: number;
  readonly staleAfterMs?: number;
  readonly reasonCode?: ReasonCode;
  readonly error?: AdminErrorResponse;
}

export interface SessionContextSource {
  loadSessionContext(options?: AdminRequestOptions): Promise<AdminSessionContext>;
  loadCapabilities(options?: AdminRequestOptions): Promise<AdminCapabilitiesResponse>;
}

export interface SessionContextLoadOptions {
  readonly previous?: SessionContextState;
  readonly traceId?: TraceId;
  readonly signatureRefs?: readonly OverridRef[];
  readonly idempotencyKey?: IdempotencyKey;
  readonly fetchedAtEpochMs?: number;
  readonly staleAfterMs?: number;
}

export const SESSION_CONTEXT_STATES: readonly SessionContextStatus[] = [
  "uninitialized",
  "loading",
  "loaded",
  "failed",
  "stale",
  "permission_denied",
];

export function createUninitializedSession(): SessionContextState {
  return { status: "uninitialized" };
}

export function createLoadingSession(previous?: SessionContextState): SessionContextState {
  return {
    status: "loading",
    context: previous?.context,
    capabilities: previous?.capabilities,
    selectedEnvironment: previous?.selectedEnvironment,
    activeTenantId: previous?.activeTenantId,
    actorId: previous?.actorId,
  };
}

export function createLoadedSession(
  context: AdminSessionContext,
  capabilities: AdminCapabilitiesResponse,
  fetchedAtEpochMs = Date.now(),
  staleAfterMs = 120000,
): SessionContextState {
  if (!hasCapability(context, "admin.session.read")) {
    return {
      status: "permission_denied",
      context,
      capabilities,
      selectedEnvironment: context.selected_environment,
      activeTenantId: context.active_tenant_id,
      actorId: context.actor_id,
      fetchedAtEpochMs,
      staleAfterMs,
      reasonCode: "admin.session.capability_missing" as ReasonCode,
    };
  }

  return {
    status: "loaded",
    context,
    capabilities,
    selectedEnvironment: context.selected_environment,
    activeTenantId: context.active_tenant_id,
    actorId: context.actor_id,
    fetchedAtEpochMs,
    staleAfterMs,
  };
}

export function createFailedSession(error: AdminErrorResponse): SessionContextState {
  const permissionDenied = error.status === 401 || error.status === 403 || error.reason_code.includes("denied");
  return {
    status: permissionDenied ? "permission_denied" : "failed",
    error,
    reasonCode: error.reason_code,
  };
}

export async function loadSessionContext(
  source: SessionContextSource,
  options: SessionContextLoadOptions = {},
): Promise<SessionContextState> {
  const requestOptions: AdminRequestOptions = {
    traceId: options.traceId,
    signatureRefs: options.signatureRefs,
    idempotencyKey: options.idempotencyKey,
  };

  try {
    const [context, capabilities] = await Promise.all([
      source.loadSessionContext(requestOptions),
      source.loadCapabilities(requestOptions),
    ]);
    return createLoadedSession(context, capabilities, options.fetchedAtEpochMs, options.staleAfterMs);
  } catch (error) {
    return createFailedSession(createSessionLoadError(error, options.traceId ?? ("trace_admin_ui_session_load" as TraceId)));
  }
}

export function markSessionStale(state: SessionContextState, nowEpochMs = Date.now()): SessionContextState {
  if (state.status !== "loaded" || state.fetchedAtEpochMs === undefined || state.staleAfterMs === undefined) {
    return state;
  }
  if (nowEpochMs - state.fetchedAtEpochMs <= state.staleAfterMs) {
    return state;
  }
  return {
    ...state,
    status: "stale",
    reasonCode: "admin.session.stale" as ReasonCode,
  };
}

export function selectEnvironment(
  context: AdminSessionContext,
  environmentId: OverridRef,
): AdminSessionContext["selected_environment"] | undefined {
  if (context.selected_environment.environment_id === environmentId) {
    return context.selected_environment;
  }
  return undefined;
}

export function hasCapability(context: AdminSessionContext, capability: AdminCapabilityFlag): boolean {
  return context.visible_capabilities.includes(capability as AdminSessionContext["visible_capabilities"][number]);
}

export function canRenderOperationalPanels(state: SessionContextState): boolean {
  return state.status === "loaded" || state.status === "stale";
}

export function getCapabilityRouteStatus(
  capabilities: AdminCapabilitiesResponse,
  capability: AdminCapabilityFlag,
): "available" | "unavailable" {
  return capabilities.routes.some((route) => route.required_capability === capability && route.available)
    ? "available"
    : "unavailable";
}

function createSessionLoadError(error: unknown, traceId: TraceId): AdminErrorResponse {
  if (isAdminErrorResponse(error)) {
    return error;
  }
  const userVisibleMessage = "Session context could not be loaded.";
  return {
    schema_version: "admin-ui-admin-api.v0.1",
    status: 500,
    reason_code: "admin.session.load_failed" as ReasonCode,
    trace_id: traceId,
    retryable: true,
    user_visible_message: userVisibleMessage,
    details_ref: "error:session_context_load" as OverridRef,
    audit_refs: [
      {
        audit_id: "audit:session_context_load_failed" as OverridRef,
        source_service: "overgate",
        trace_id: traceId,
      },
    ],
  };
}

function isAdminErrorResponse(error: unknown): error is AdminErrorResponse {
  if (!error || typeof error !== "object") {
    return false;
  }
  const candidate = error as Partial<AdminErrorResponse>;
  return (
    typeof candidate.schema_version === "string" &&
    typeof candidate.status === "number" &&
    typeof candidate.reason_code === "string" &&
    typeof candidate.trace_id === "string" &&
    typeof candidate.retryable === "boolean" &&
    typeof candidate.user_visible_message === "string" &&
    typeof candidate.details_ref === "string" &&
    Array.isArray(candidate.audit_refs)
  );
}
