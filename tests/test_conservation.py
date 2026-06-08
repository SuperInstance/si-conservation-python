"""Tests for si_conservation Python bindings."""
import si_conservation
import pytest


# ── Budget creation and properties ──────────────────────────────────

class TestBudgetCreation:
    def test_basic_creation(self):
        b = si_conservation.Budget(gamma=143, eta=82)
        assert b.gamma == 143
        assert b.eta == 82

    def test_total(self):
        b = si_conservation.Budget(gamma=143, eta=82)
        assert b.total == 225

    def test_zero_budget(self):
        b = si_conservation.Budget(gamma=0, eta=0)
        assert b.gamma == 0
        assert b.eta == 0
        assert b.total == 0

    def test_large_values(self):
        b = si_conservation.Budget(gamma=2**32, eta=2**32)
        assert b.total == 2**33

    def test_gamma_only(self):
        b = si_conservation.Budget(gamma=100, eta=0)
        assert b.total == 100

    def test_eta_only(self):
        b = si_conservation.Budget(gamma=0, eta=100)
        assert b.total == 100


class TestBudgetConservation:
    def test_check_true(self):
        b = si_conservation.Budget(gamma=100, eta=50)
        assert b.check() is True

    def test_check_zero(self):
        b = si_conservation.Budget(gamma=0, eta=0)
        assert b.check() is True

    def test_check_large(self):
        b = si_conservation.Budget(gamma=1_000_000, eta=999_999)
        assert b.check() is True


class TestBudgetEquality:
    def test_equal_budgets(self):
        b1 = si_conservation.Budget(gamma=100, eta=50)
        b2 = si_conservation.Budget(gamma=100, eta=50)
        assert b1 == b2

    def test_unequal_budgets(self):
        b1 = si_conservation.Budget(gamma=100, eta=50)
        b2 = si_conservation.Budget(gamma=99, eta=50)
        assert b1 != b2

    def test_unequal_eta(self):
        b1 = si_conservation.Budget(gamma=100, eta=50)
        b2 = si_conservation.Budget(gamma=100, eta=51)
        assert b1 != b2


class TestBudgetRepr:
    def test_repr(self):
        b = si_conservation.Budget(gamma=143, eta=82)
        r = repr(b)
        assert "Budget" in r
        assert "143" in r
        assert "82" in r
        assert "225" in r


# ── FleetBudget ─────────────────────────────────────────────────────

class TestFleetBudget:
    def test_basic_fleet(self):
        fleet = si_conservation.FleetBudget([
            si_conservation.Budget(gamma=100, eta=50),
            si_conservation.Budget(gamma=43, eta=32),
        ])
        assert fleet.total_gamma == 143
        assert fleet.total_eta == 82
        assert fleet.total == 225

    def test_invariant_holds(self):
        fleet = si_conservation.FleetBudget([
            si_conservation.Budget(gamma=100, eta=50),
            si_conservation.Budget(gamma=43, eta=32),
        ])
        assert fleet.invariant_holds() is True

    def test_empty_fleet(self):
        fleet = si_conservation.FleetBudget([])
        assert fleet.total_gamma == 0
        assert fleet.total_eta == 0
        assert fleet.total == 0
        assert fleet.invariant_holds() is True

    def test_fleet_count(self):
        fleet = si_conservation.FleetBudget([
            si_conservation.Budget(gamma=1, eta=2),
            si_conservation.Budget(gamma=3, eta=4),
            si_conservation.Budget(gamma=5, eta=6),
        ])
        assert fleet.count == 3
        assert len(fleet) == 3

    def test_single_node_fleet(self):
        fleet = si_conservation.FleetBudget([
            si_conservation.Budget(gamma=500, eta=250),
        ])
        assert fleet.count == 1
        assert fleet.total_gamma == 500
        assert fleet.total_eta == 250

    def test_fleet_repr(self):
        fleet = si_conservation.FleetBudget([
            si_conservation.Budget(gamma=100, eta=50),
        ])
        r = repr(fleet)
        assert "FleetBudget" in r
        assert "100" in r


# ── ConservationGauge ───────────────────────────────────────────────

class TestConservationGauge:
    def test_gamma_fraction(self):
        fleet = si_conservation.FleetBudget([
            si_conservation.Budget(gamma=143, eta=82),
        ])
        gauge = si_conservation.ConservationGauge(fleet)
        assert abs(gauge.gamma_fraction() - 143 / 225) < 1e-10

    def test_eta_fraction(self):
        fleet = si_conservation.FleetBudget([
            si_conservation.Budget(gamma=143, eta=82),
        ])
        gauge = si_conservation.ConservationGauge(fleet)
        assert abs(gauge.eta_fraction() - 82 / 225) < 1e-10

    def test_fractions_sum_to_one(self):
        fleet = si_conservation.FleetBudget([
            si_conservation.Budget(gamma=100, eta=50),
            si_conservation.Budget(gamma=43, eta=32),
        ])
        gauge = si_conservation.ConservationGauge(fleet)
        assert gauge.fractions_valid() is True

    def test_empty_fleet_gauge(self):
        fleet = si_conservation.FleetBudget([])
        gauge = si_conservation.ConservationGauge(fleet)
        assert gauge.gamma_fraction() == 0.0
        assert gauge.eta_fraction() == 0.0

    def test_all_gamma(self):
        fleet = si_conservation.FleetBudget([
            si_conservation.Budget(gamma=100, eta=0),
        ])
        gauge = si_conservation.ConservationGauge(fleet)
        assert gauge.gamma_fraction() == 1.0
        assert gauge.eta_fraction() == 0.0

    def test_all_eta(self):
        fleet = si_conservation.FleetBudget([
            si_conservation.Budget(gamma=0, eta=100),
        ])
        gauge = si_conservation.ConservationGauge(fleet)
        assert gauge.gamma_fraction() == 0.0
        assert gauge.eta_fraction() == 1.0

    def test_gauge_repr(self):
        fleet = si_conservation.FleetBudget([
            si_conservation.Budget(gamma=100, eta=50),
        ])
        gauge = si_conservation.ConservationGauge(fleet)
        r = repr(gauge)
        assert "ConservationGauge" in r


# ── check_budgets helper ────────────────────────────────────────────

class TestCheckBudgets:
    def test_all_valid(self):
        budgets = [
            si_conservation.Budget(gamma=100, eta=50),
            si_conservation.Budget(gamma=43, eta=32),
        ]
        assert si_conservation.check_budgets(budgets) is True

    def test_empty_list(self):
        assert si_conservation.check_budgets([]) is True

    def test_single_budget(self):
        assert si_conservation.check_budgets([
            si_conservation.Budget(gamma=1, eta=1),
        ]) is True
