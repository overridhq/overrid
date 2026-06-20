import type { AdminDependencyStatus, AuditRef, ReasonCode, TraceId, UiDiagnosticEvent } from "./contracts";

export type RenderedFieldKind = "value" | "redacted" | "unavailable" | "denied" | "stale" | "degraded";

export interface RenderFieldInput {
  readonly fieldName: string;
  readonly value?: string | number | boolean | null;
  readonly redacted?: boolean;
  readonly denied?: boolean;
  readonly stale?: boolean;
  readonly dependencyStatus?: AdminDependencyStatus;
  readonly reasonCodes?: readonly ReasonCode[];
}

export interface RenderedField {
  readonly kind: RenderedFieldKind;
  readonly label: string;
  readonly displayValue: string;
  readonly reasonCodes: readonly ReasonCode[];
}

const PRIVATE_FIELD_HINTS = [
  "key_material",
  "signature_refs",
  "credential",
  "private_payload",
  "decrypted",
  "prompt",
  "raw_payload",
];

export function renderAdminField(input: RenderFieldInput): RenderedField {
  const reasonCodes = input.reasonCodes ?? [];

  if (containsPrivateFieldHint(input.fieldName)) {
    return {
      kind: "redacted",
      label: input.fieldName,
      displayValue: "redacted",
      reasonCodes: ["admin.redaction.private_field" as ReasonCode, ...reasonCodes],
    };
  }
  if (input.redacted) {
    return { kind: "redacted", label: input.fieldName, displayValue: "redacted", reasonCodes };
  }
  if (input.denied) {
    return { kind: "denied", label: input.fieldName, displayValue: "denied", reasonCodes };
  }
  if (input.stale) {
    return { kind: "stale", label: input.fieldName, displayValue: formatPrimitive(input.value), reasonCodes };
  }
  if (input.dependencyStatus?.status === "unavailable") {
    return {
      kind: "unavailable",
      label: input.fieldName,
      displayValue: "unavailable",
      reasonCodes: [input.dependencyStatus.reason_code, ...reasonCodes],
    };
  }
  if (input.dependencyStatus?.status === "degraded" || input.dependencyStatus?.status === "redacted") {
    return {
      kind: "degraded",
      label: input.fieldName,
      displayValue: input.dependencyStatus.status,
      reasonCodes: [input.dependencyStatus.reason_code, ...reasonCodes],
    };
  }
  if (input.value === undefined || input.value === null || input.value === "") {
    return { kind: "unavailable", label: input.fieldName, displayValue: "unavailable", reasonCodes };
  }
  return { kind: "value", label: input.fieldName, displayValue: formatPrimitive(input.value), reasonCodes };
}

export function formatReasonCode(reasonCode: ReasonCode): string {
  return reasonCode.split(".").join(" / ");
}

export function formatAuditRef(auditRef: AuditRef): string {
  return `${auditRef.source_service}:${auditRef.audit_id}:${auditRef.trace_id}`;
}

export function copySafeDiagnosticRef(event: UiDiagnosticEvent): string {
  return [event.trace_id, event.reason_code, ...event.safe_refs].join(" | ");
}

export function createLocalDiagnosticEvent(
  traceId: TraceId,
  reasonCode: ReasonCode,
  message: string,
  safeRefs: readonly string[],
): Pick<UiDiagnosticEvent, "trace_id" | "reason_code" | "message" | "safe_refs"> {
  return {
    trace_id: traceId,
    reason_code: reasonCode,
    message,
    safe_refs: [...safeRefs] as UiDiagnosticEvent["safe_refs"],
  };
}

export function containsPrivateFieldHint(fieldName: string): boolean {
  const normalized = fieldName.toLowerCase();
  return PRIVATE_FIELD_HINTS.some((hint) => normalized.includes(hint));
}

function formatPrimitive(value: string | number | boolean | null | undefined): string {
  if (value === undefined || value === null) {
    return "unavailable";
  }
  return String(value);
}
