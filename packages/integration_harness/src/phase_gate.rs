#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateClass {
    Smoke,
    ContractSpine,
    Regression,
    Extended,
    ReleaseCandidate,
}

impl GateClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Smoke => "smoke",
            Self::ContractSpine => "contract_spine",
            Self::Regression => "regression",
            Self::Extended => "extended",
            Self::ReleaseCandidate => "release_candidate",
        }
    }
}

pub fn mandatory_gate_classes(master_phase: u8) -> &'static [GateClass] {
    match master_phase {
        0 => &[GateClass::Smoke, GateClass::ContractSpine],
        1 | 2 => &[GateClass::Smoke, GateClass::ContractSpine],
        3..=12 => &[
            GateClass::Smoke,
            GateClass::ContractSpine,
            GateClass::Regression,
        ],
        13 => &[GateClass::Regression, GateClass::ReleaseCandidate],
        _ => &[],
    }
}

pub fn scenario_matches_phase_filter(scenario_phase: u8, phase_filter: Option<u8>) -> bool {
    match phase_filter {
        Some(phase) => scenario_phase == phase,
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_filter_is_exact_and_optional() {
        assert!(scenario_matches_phase_filter(0, None));
        assert!(scenario_matches_phase_filter(0, Some(0)));
        assert!(!scenario_matches_phase_filter(0, Some(3)));
    }

    #[test]
    fn phase3_includes_regression_gate_class() {
        let gates = mandatory_gate_classes(3)
            .iter()
            .map(|gate| gate.as_str())
            .collect::<Vec<_>>();
        assert_eq!(gates, vec!["smoke", "contract_spine", "regression"]);
    }
}
