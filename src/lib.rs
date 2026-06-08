use pyo3::prelude::*;

mod conservation;
mod types;

/// SuperInstance conservation law Python bindings.
///
/// Provides Budget, FleetBudget, and ConservationGauge types
/// for verifying the invariant: γ + η = total_budget.
#[pymodule]
fn si_conservation(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<types::PyBudget>()?;
    m.add_class::<types::PyFleetBudget>()?;
    m.add_class::<types::PyConservationGauge>()?;
    m.add_function(wrap_pyfunction!(types::check_budgets, m)?)?;

    // Python-friendly aliases (drop the Py prefix)
    m.add("Budget", m.getattr("PyBudget")?)?;
    m.add("FleetBudget", m.getattr("PyFleetBudget")?)?;
    m.add("ConservationGauge", m.getattr("PyConservationGauge")?)?;
    Ok(())
}
