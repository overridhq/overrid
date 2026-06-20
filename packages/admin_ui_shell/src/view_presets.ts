import type { OverridRef, ReasonCode } from "./contracts";
import { OPERATOR_PANEL_IDS, type PanelId } from "./operator_shell";

export type OperatorViewPresetVersion = "operator-view-preset.v0.1";
export type OperatorViewDensity = "compact" | "comfortable";
export type SortDirection = "asc" | "desc";

export interface OperatorViewPresetScope {
  readonly actorId: OverridRef;
  readonly tenantId: OverridRef;
  readonly viewId: string;
}

export interface OperatorViewPreset {
  readonly schema_version: OperatorViewPresetVersion;
  readonly actor_id: OverridRef;
  readonly tenant_id: OverridRef;
  readonly view_id: string;
  readonly panel_order: readonly PanelId[];
  readonly columns_by_panel: Readonly<Partial<Record<PanelId, readonly string[]>>>;
  readonly filters_by_panel: Readonly<Partial<Record<PanelId, Readonly<Record<string, string>>>>>;
  readonly sort_by_panel: Readonly<Partial<Record<PanelId, { readonly field: string; readonly direction: SortDirection }>>>;
  readonly density: OperatorViewDensity;
  readonly updated_at: string;
}

export interface PresetValidationResult {
  readonly ok: boolean;
  readonly reasonCodes: readonly ReasonCode[];
}

export interface OperatorPresetStorage {
  read(key: string): string | null;
  write(key: string, value: string): void;
  remove(key: string): void;
}

const FORBIDDEN_PRESET_PATTERNS = [
  /\bpassword\s*=/i,
  /\bsecret\b/i,
  /\bcredential\b/i,
  /\bprivate[_ -]?payload\b/i,
  /\bdecrypted\b/i,
  /\bprompt\b/i,
  /\/Users\/|\/home\/|[A-Za-z]:\\/,
];

const OPERATOR_PANEL_ID_SET = new Set<string>(OPERATOR_PANEL_IDS);

export function scopePresetKey(scope: OperatorViewPresetScope): string {
  return ["operator_view_preset", scope.actorId, scope.tenantId, scope.viewId].map(encodeURIComponent).join(":");
}

export function createDefaultOperatorViewPreset(scope: OperatorViewPresetScope, panelOrder: readonly PanelId[]): OperatorViewPreset {
  return {
    schema_version: "operator-view-preset.v0.1",
    actor_id: scope.actorId,
    tenant_id: scope.tenantId,
    view_id: scope.viewId,
    panel_order: [...panelOrder],
    columns_by_panel: {},
    filters_by_panel: {},
    sort_by_panel: {},
    density: "compact",
    updated_at: new Date(0).toISOString(),
  };
}

export function validateOperatorViewPreset(value: unknown, expectedScope?: OperatorViewPresetScope): PresetValidationResult {
  const reasonCodes: ReasonCode[] = [];
  if (!isRecord(value)) {
    return { ok: false, reasonCodes: ["admin.preset.invalid_shape" as ReasonCode] };
  }

  const preset = value as Partial<OperatorViewPreset>;
  if (preset.schema_version !== "operator-view-preset.v0.1") {
    reasonCodes.push("admin.preset.schema_version_invalid" as ReasonCode);
  }
  if (!isNonEmptyString(preset.actor_id) || !isNonEmptyString(preset.tenant_id) || !isNonEmptyString(preset.view_id)) {
    reasonCodes.push("admin.preset.scope_missing" as ReasonCode);
  }
  if (expectedScope && !isPresetInScope(preset, expectedScope)) {
    reasonCodes.push("admin.preset.scope_mismatch" as ReasonCode);
  }
  if (!Array.isArray(preset.panel_order) || preset.panel_order.length === 0) {
    reasonCodes.push("admin.preset.panel_order_invalid" as ReasonCode);
  } else {
    if (new Set(preset.panel_order).size !== preset.panel_order.length) {
      reasonCodes.push("admin.preset.panel_order_duplicate" as ReasonCode);
    }
    if (!preset.panel_order.every(isKnownPanelId)) {
      reasonCodes.push("admin.preset.panel_unknown" as ReasonCode);
    }
  }
  if (!isRecord(preset.columns_by_panel) || !isRecord(preset.filters_by_panel) || !isRecord(preset.sort_by_panel)) {
    reasonCodes.push("admin.preset.layout_invalid" as ReasonCode);
  } else if (
    !hasOnlyKnownPanelKeys(preset.columns_by_panel) ||
    !hasOnlyKnownPanelKeys(preset.filters_by_panel) ||
    !hasOnlyKnownPanelKeys(preset.sort_by_panel)
  ) {
    reasonCodes.push("admin.preset.panel_key_unknown" as ReasonCode);
  }
  if (preset.density !== "compact" && preset.density !== "comfortable") {
    reasonCodes.push("admin.preset.density_invalid" as ReasonCode);
  }
  if (containsForbiddenPresetText(value)) {
    reasonCodes.push("admin.preset.private_content" as ReasonCode);
  }

  return { ok: reasonCodes.length === 0, reasonCodes };
}

export function loadScopedPreset(storage: OperatorPresetStorage, scope: OperatorViewPresetScope): OperatorViewPreset | null {
  const key = scopePresetKey(scope);
  const stored = storage.read(key);
  if (!stored) {
    return null;
  }

  try {
    const parsed = JSON.parse(stored) as unknown;
    const validation = validateOperatorViewPreset(parsed, scope);
    if (!validation.ok) {
      storage.remove(key);
      return null;
    }
    return parsed as OperatorViewPreset;
  } catch {
    storage.remove(key);
    return null;
  }
}

export function saveScopedPreset(storage: OperatorPresetStorage, preset: OperatorViewPreset): void {
  const scope = { actorId: preset.actor_id, tenantId: preset.tenant_id, viewId: preset.view_id };
  const validation = validateOperatorViewPreset(preset, scope);
  if (!validation.ok) {
    throw new Error(`Invalid operator view preset: ${validation.reasonCodes.join(",")}`);
  }
  storage.write(scopePresetKey(scope), JSON.stringify(preset));
}

export function resetScopedPreset(storage: OperatorPresetStorage, scope: OperatorViewPresetScope): void {
  storage.remove(scopePresetKey(scope));
}

export function isPresetInScope(
  preset: Partial<Pick<OperatorViewPreset, "actor_id" | "tenant_id" | "view_id">>,
  scope: OperatorViewPresetScope,
): boolean {
  return preset.actor_id === scope.actorId && preset.tenant_id === scope.tenantId && preset.view_id === scope.viewId;
}

export function containsForbiddenPresetText(value: unknown): boolean {
  if (typeof value === "string") {
    return FORBIDDEN_PRESET_PATTERNS.some((pattern) => pattern.test(value));
  }
  if (Array.isArray(value)) {
    return value.some(containsForbiddenPresetText);
  }
  if (isRecord(value)) {
    return Object.entries(value).some(
      ([key, nestedValue]) => containsForbiddenPresetText(key) || containsForbiddenPresetText(nestedValue),
    );
  }
  return false;
}

function hasOnlyKnownPanelKeys(value: Record<string, unknown>): boolean {
  return Object.keys(value).every(isKnownPanelId);
}

function isKnownPanelId(value: unknown): value is PanelId {
  return typeof value === "string" && OPERATOR_PANEL_ID_SET.has(value);
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function isNonEmptyString(value: unknown): value is string {
  return typeof value === "string" && value.length > 0;
}
