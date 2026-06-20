pub const DEFAULT_FIXTURE_SEED: &str = "seed_phase0_smoke_0001";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicFixtureSeed {
    pub scenario_id: String,
    pub seed: String,
    pub fixture_id: String,
}

impl DeterministicFixtureSeed {
    pub fn new(scenario_id: impl Into<String>, seed: impl Into<String>) -> Self {
        let scenario_id = scenario_id.into();
        let seed = seed.into();
        let fixture_id = format!("fixture_{}", stable_token(&[&scenario_id, &seed]));
        Self {
            scenario_id,
            seed,
            fixture_id,
        }
    }
}

pub fn stable_token(parts: &[&str]) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

pub fn stable_short_token(parts: &[&str]) -> String {
    stable_token(parts).chars().take(12).collect()
}

pub fn sanitize_identifier(value: &str) -> String {
    let sanitized = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    let compact = sanitized
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_");
    if compact.is_empty() {
        "value".to_owned()
    } else {
        compact
    }
}

pub fn fixture_id_from_ref(value: &str) -> String {
    if let Some(rest) = value.strip_prefix("fixture:") {
        format!("fixture_{}", sanitize_identifier(rest))
    } else {
        sanitize_identifier(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_seed_is_deterministic() {
        let first = DeterministicFixtureSeed::new("scenario_phase0_smoke", DEFAULT_FIXTURE_SEED);
        let second = DeterministicFixtureSeed::new("scenario_phase0_smoke", DEFAULT_FIXTURE_SEED);
        assert_eq!(first, second);
        assert!(first.fixture_id.starts_with("fixture_"));
    }

    #[test]
    fn fixture_refs_normalize_to_fixture_ids() {
        assert_eq!(
            fixture_id_from_ref("fixture:phase0_smoke"),
            "fixture_phase0_smoke"
        );
        assert_eq!(fixture_id_from_ref("fixture_phase0_smoke"), "fixture_phase0_smoke");
    }
}
