pub const OVERKEY_PHASE2_RESPONSE_SCHEMA_VERSION: &str = "overkey.phase2.response.v0";
pub const OVERKEY_PHASE3_RESPONSE_SCHEMA_VERSION: &str = "overkey.phase3.response.v0";
pub const OVERKEY_PHASE4_RESPONSE_SCHEMA_VERSION: &str = "overkey.phase4.response.v0";
pub const OVERKEY_PHASE5_RESPONSE_SCHEMA_VERSION: &str = "overkey.phase5.response.v0";
pub const OVERKEY_PHASE6_RESPONSE_SCHEMA_VERSION: &str = "overkey.phase6.response.v0";
pub const OVERKEY_PHASE2_FIXTURE_SCHEMA_VERSION: &str = "overkey.phase2.local_fixture.v0";
pub const OVERKEY_PHASE3_FIXTURE_SCHEMA_VERSION: &str = "overkey.phase3.credential_enrollment.v0";
pub const OVERKEY_PHASE4_FIXTURE_SCHEMA_VERSION: &str = "overkey.phase4.verification.v0";
pub const OVERKEY_PHASE5_FIXTURE_SCHEMA_VERSION: &str = "overkey.phase5.lifecycle.v0";
pub const OVERKEY_PHASE6_FIXTURE_SCHEMA_VERSION: &str =
    "overkey.phase6.delegation_policy_usage.v0";
pub const OVERKEY_SCHEMA_SOURCE: &str =
    "packages/schemas/overrid_contracts/v0/overkey_credential.schema.json";

pub const CREDENTIAL_RECORD_SCHEMA_REF: &str = "schema:overkey:credential_record:v0";
pub const API_KEY_RECORD_SCHEMA_REF: &str = "schema:overkey:api_key_record:v0";
pub const PUBLIC_KEY_RECORD_SCHEMA_REF: &str = "schema:overkey:public_key_record:v0";
pub const SERVICE_ACCOUNT_KEY_SCHEMA_REF: &str = "schema:overkey:service_account_key:v0";
pub const DELEGATION_RECORD_SCHEMA_REF: &str = "schema:overkey:delegation_record:v0";
pub const ROTATION_RECORD_SCHEMA_REF: &str = "schema:overkey:rotation_record:v0";
pub const REVOCATION_RECORD_SCHEMA_REF: &str = "schema:overkey:revocation_record:v0";
pub const VERIFICATION_RESULT_SCHEMA_REF: &str = "schema:overkey:verification_result:v0";
pub const SECRET_REF_SCHEMA_REF: &str = "schema:overkey:secret_ref:v0";
pub const API_ERROR_SCHEMA_REF: &str = "schema:overkey:api_error:v0";
pub const OVERWATCH_EVENT_SCHEMA_REF: &str = "schema:overkey:overwatch_event:v0";

pub const REQUIRED_SCHEMA_OBJECTS: [&str; 11] = [
    "credential_record",
    "api_key_record",
    "public_key_record",
    "service_account_key",
    "delegation_record",
    "rotation_record",
    "revocation_record",
    "verification_result",
    "secret_ref",
    "api_error",
    "overwatch_event",
];

pub const RAW_SECRET_SENTINELS: [&str; 8] = [
    "raw_api_key=",
    "api_key=",
    "private_key=",
    "private key",
    "-----begin",
    "-----BEGIN",
    "seed_phrase=",
    "password=",
];

pub fn schema_ref_for(record_kind: &str) -> Option<&'static str> {
    match record_kind {
        "credential_record" => Some(CREDENTIAL_RECORD_SCHEMA_REF),
        "api_key_record" => Some(API_KEY_RECORD_SCHEMA_REF),
        "public_key_record" => Some(PUBLIC_KEY_RECORD_SCHEMA_REF),
        "service_account_key" => Some(SERVICE_ACCOUNT_KEY_SCHEMA_REF),
        "delegation_record" => Some(DELEGATION_RECORD_SCHEMA_REF),
        "rotation_record" => Some(ROTATION_RECORD_SCHEMA_REF),
        "revocation_record" => Some(REVOCATION_RECORD_SCHEMA_REF),
        "verification_result" => Some(VERIFICATION_RESULT_SCHEMA_REF),
        "secret_ref" => Some(SECRET_REF_SCHEMA_REF),
        "api_error" => Some(API_ERROR_SCHEMA_REF),
        "overwatch_event" => Some(OVERWATCH_EVENT_SCHEMA_REF),
        _ => None,
    }
}

pub fn contains_raw_secret_marker(serialized: &str) -> bool {
    let lowered = serialized.to_ascii_lowercase();
    RAW_SECRET_SENTINELS
        .iter()
        .any(|marker| lowered.contains(&marker.to_ascii_lowercase()))
}
