/// Core conservation law logic.
///
/// The SuperInstance conservation law states that for any budget allocation:
///     γ (gamma) + η (eta) = total_budget
///
/// This must hold at every level: individual nodes, fleets, and globally.

/// A single budget allocation with gamma and eta components.
#[derive(Debug, Clone)]
pub struct Budget {
    pub gamma: u64,
    pub eta: u64,
}

impl Budget {
    pub fn new(gamma: u64, eta: u64) -> Self {
        Self { gamma, eta }
    }

    /// Total budget = gamma + eta.
    pub fn total(&self) -> u64 {
        self.gamma + self.eta
    }

    /// Check that the conservation law holds.
    /// Always true by construction (total is derived), but included for API completeness.
    pub fn check(&self) -> bool {
        self.gamma + self.eta == self.total()
    }
}

/// A fleet-level budget aggregating multiple individual budgets.
#[derive(Debug, Clone)]
pub struct FleetBudget {
    pub budgets: Vec<Budget>,
}

impl FleetBudget {
    pub fn new(budgets: Vec<Budget>) -> Self {
        Self { budgets }
    }

    /// Sum of all gamma values across the fleet.
    pub fn total_gamma(&self) -> u64 {
        self.budgets.iter().map(|b| b.gamma).sum()
    }

    /// Sum of all eta values across the fleet.
    pub fn total_eta(&self) -> u64 {
        self.budgets.iter().map(|b| b.eta).sum()
    }

    /// Total fleet budget.
    pub fn total(&self) -> u64 {
        self.total_gamma() + self.total_eta()
    }

    /// Verify the conservation invariant holds across the fleet.
    pub fn invariant_holds(&self) -> bool {
        self.total_gamma() + self.total_eta() == self.total()
    }

    /// Number of budgets in the fleet.
    pub fn len(&self) -> usize {
        self.budgets.len()
    }

    /// Whether the fleet is empty.
    pub fn is_empty(&self) -> bool {
        self.budgets.is_empty()
    }
}

/// A conservation gauge that measures budget proportions.
#[derive(Debug, Clone)]
pub struct ConservationGauge {
    fleet: FleetBudget,
}

impl ConservationGauge {
    pub fn new(fleet: FleetBudget) -> Self {
        Self { fleet }
    }

    /// Fraction of total budget allocated to gamma.
    /// Returns 0.0 if the fleet has zero total budget.
    pub fn gamma_fraction(&self) -> f64 {
        let total = self.fleet.total();
        if total == 0 {
            return 0.0;
        }
        self.fleet.total_gamma() as f64 / total as f64
    }

    /// Fraction of total budget allocated to eta.
    /// Returns 0.0 if the fleet has zero total budget.
    pub fn eta_fraction(&self) -> f64 {
        let total = self.fleet.total();
        if total == 0 {
            return 0.0;
        }
        self.fleet.total_eta() as f64 / total as f64
    }

    /// Verify that fractions sum to 1.0 (within floating point tolerance).
    pub fn fractions_valid(&self) -> bool {
        let sum = self.gamma_fraction() + self.eta_fraction();
        (sum - 1.0).abs() < 1e-10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_basic() {
        let b = Budget::new(143, 82);
        assert_eq!(b.total(), 225);
        assert!(b.check());
    }

    #[test]
    fn test_budget_zero() {
        let b = Budget::new(0, 0);
        assert_eq!(b.total(), 0);
        assert!(b.check());
    }

    #[test]
    fn test_fleet_basic() {
        let fleet = FleetBudget::new(vec![
            Budget::new(100, 50),
            Budget::new(43, 32),
        ]);
        assert_eq!(fleet.total_gamma(), 143);
        assert_eq!(fleet.total_eta(), 82);
        assert_eq!(fleet.total(), 225);
        assert!(fleet.invariant_holds());
        assert_eq!(fleet.len(), 2);
    }

    #[test]
    fn test_gauge_fractions() {
        let fleet = FleetBudget::new(vec![
            Budget::new(100, 50),
            Budget::new(43, 32),
        ]);
        let gauge = ConservationGauge::new(fleet);
        assert!((gauge.gamma_fraction() - 143.0 / 225.0).abs() < 1e-10);
        assert!((gauge.eta_fraction() - 82.0 / 225.0).abs() < 1e-10);
        assert!(gauge.fractions_valid());
    }

    #[test]
    fn test_gauge_zero_fleet() {
        let fleet = FleetBudget::new(vec![]);
        let gauge = ConservationGauge::new(fleet);
        assert_eq!(gauge.gamma_fraction(), 0.0);
        assert_eq!(gauge.eta_fraction(), 0.0);
    }
}
