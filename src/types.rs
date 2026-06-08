use pyo3::prelude::*;
use crate::conservation;

/// A single budget allocation with gamma (γ) and eta (η) components.
///
/// The conservation law states: gamma + eta == total.
///
/// Example:
///     >>> b = Budget(gamma=143, eta=82)
///     >>> b.total
///     225
///     >>> b.check()
///     True
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyBudget {
    inner: conservation::Budget,
}

#[pymethods]
impl PyBudget {
    #[new]
    #[pyo3(signature = (gamma, eta))]
    fn new(gamma: u64, eta: u64) -> Self {
        Self {
            inner: conservation::Budget::new(gamma, eta),
        }
    }

    /// Gamma (γ) component of the budget.
    #[getter]
    fn gamma(&self) -> u64 {
        self.inner.gamma
    }

    /// Eta (η) component of the budget.
    #[getter]
    fn eta(&self) -> u64 {
        self.inner.eta
    }

    /// Total budget: gamma + eta.
    #[getter]
    fn total(&self) -> u64 {
        self.inner.total()
    }

    /// Check that the conservation law holds.
    fn check(&self) -> bool {
        self.inner.check()
    }

    fn __repr__(&self) -> String {
        format!("Budget(gamma={}, eta={}, total={})", self.inner.gamma, self.inner.eta, self.inner.total())
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner.gamma == other.inner.gamma && self.inner.eta == other.inner.eta
    }
}

impl PyBudget {
    pub fn inner(&self) -> &conservation::Budget {
        &self.inner
    }
}

/// A fleet-level budget aggregating multiple individual budgets.
///
/// Example:
///     >>> fleet = FleetBudget([Budget(100, 50), Budget(43, 32)])
///     >>> fleet.total_gamma
///     143
///     >>> fleet.total_eta
///     82
///     >>> fleet.invariant_holds()
///     True
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyFleetBudget {
    inner: conservation::FleetBudget,
}

#[pymethods]
impl PyFleetBudget {
    #[new]
    fn new(budgets: Vec<PyBudget>) -> Self {
        let inner_budgets: Vec<conservation::Budget> = budgets.into_iter().map(|b| b.inner.clone()).collect();
        Self {
            inner: conservation::FleetBudget::new(inner_budgets),
        }
    }

    /// Sum of all gamma values across the fleet.
    #[getter]
    fn total_gamma(&self) -> u64 {
        self.inner.total_gamma()
    }

    /// Sum of all eta values across the fleet.
    #[getter]
    fn total_eta(&self) -> u64 {
        self.inner.total_eta()
    }

    /// Total fleet budget.
    #[getter]
    fn total(&self) -> u64 {
        self.inner.total()
    }

    /// Verify the conservation invariant holds across the fleet.
    fn invariant_holds(&self) -> bool {
        self.inner.invariant_holds()
    }

    /// Number of budgets in the fleet.
    #[getter]
    fn count(&self) -> usize {
        self.inner.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "FleetBudget(count={}, total_gamma={}, total_eta={}, total={})",
            self.inner.len(),
            self.inner.total_gamma(),
            self.inner.total_eta(),
            self.inner.total()
        )
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}

/// A conservation gauge that measures budget proportions.
///
/// Example:
///     >>> fleet = FleetBudget([Budget(143, 82)])
///     >>> gauge = ConservationGauge(fleet)
///     >>> gauge.gamma_fraction()
///     0.635...
///     >>> gauge.eta_fraction()
///     0.364...
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyConservationGauge {
    inner: conservation::ConservationGauge,
}

#[pymethods]
impl PyConservationGauge {
    #[new]
    fn new(fleet: &PyFleetBudget) -> Self {
        Self {
            inner: conservation::ConservationGauge::new(fleet.inner.clone()),
        }
    }

    /// Fraction of total budget allocated to gamma.
    fn gamma_fraction(&self) -> f64 {
        self.inner.gamma_fraction()
    }

    /// Fraction of total budget allocated to eta.
    fn eta_fraction(&self) -> f64 {
        self.inner.eta_fraction()
    }

    /// Verify that gamma_fraction + eta_fraction == 1.0.
    fn fractions_valid(&self) -> bool {
        self.inner.fractions_valid()
    }

    fn __repr__(&self) -> String {
        format!(
            "ConservationGauge(gamma={:.4}, eta={:.4})",
            self.inner.gamma_fraction(),
            self.inner.eta_fraction()
        )
    }
}

/// Convenience function: check multiple budgets at once.
///
/// Returns True if every budget satisfies the conservation law.
///
/// Example:
///     >>> check_budgets([Budget(100, 50), Budget(43, 32)])
///     True
#[pyfunction]
pub fn check_budgets(budgets: Vec<PyBudget>) -> bool {
    budgets.iter().all(|b| b.inner.check())
}
