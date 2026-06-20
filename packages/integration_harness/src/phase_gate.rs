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

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "smoke" => Some(Self::Smoke),
            "contract_spine" => Some(Self::ContractSpine),
            "regression" => Some(Self::Regression),
            "extended" => Some(Self::Extended),
            "release_candidate" => Some(Self::ReleaseCandidate),
            _ => None,
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
        Some(phase) => scenario_phase <= phase,
        None => true,
    }
}

pub fn gate_class_is_mandatory(gate_class: &str, requested_phase: u8) -> bool {
    let Some(gate_class) = GateClass::parse(gate_class) else {
        return false;
    };
    mandatory_gate_classes(requested_phase).contains(&gate_class)
}

pub fn scenario_is_selected_for_phase(
    scenario_phase: u8,
    gate_class: &str,
    phase_filter: Option<u8>,
) -> bool {
    match phase_filter {
        Some(phase) => scenario_phase <= phase && gate_class_is_mandatory(gate_class, phase),
        None => true,
    }
}

pub fn scenario_is_planned_for_phase(
    scenario_phase: u8,
    gate_class: &str,
    phase_filter: Option<u8>,
) -> bool {
    match phase_filter {
        Some(phase) => scenario_phase > phase || !gate_class_is_mandatory(gate_class, phase),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_filter_is_inherited_and_optional() {
        assert!(scenario_matches_phase_filter(0, None));
        assert!(scenario_matches_phase_filter(0, Some(0)));
        assert!(scenario_matches_phase_filter(0, Some(3)));
        assert!(!scenario_matches_phase_filter(3, Some(0)));
    }

    #[test]
    fn selection_requires_inherited_phase_and_mandatory_gate() {
        assert!(scenario_is_selected_for_phase(0, "smoke", Some(1)));
        assert!(scenario_is_selected_for_phase(1, "contract_spine", Some(1)));
        assert!(!scenario_is_selected_for_phase(6, "extended", Some(6)));
        assert!(scenario_is_planned_for_phase(6, "extended", Some(6)));
        assert!(scenario_is_planned_for_phase(13, "extended", Some(1)));
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
