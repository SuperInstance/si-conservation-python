# si-conservation-python

**PyO3 Python bindings for the SuperInstance conservation law.** Verify the invariant **γ + η = total** from Python with native Rust performance. Built with [maturin](https://www.maturin.rs/).

---

## The Conservation Law

The SuperInstance conservation law states that for any budget allocation:

```
γ (gamma) + η (eta) = total_budget
```

- **γ (gamma)** — productive energy: tokens spent on useful output
- **η (eta)** — entropy/waste: tokens burned on errors, retries, hallucinations
- **total_budget** — the compute ceiling (fixed)

This invariant must hold at every level: individual agents, fleets, and globally.

---

## Installation

### From PyPI (when published)

```bash
pip install si-conservation
```

### Build from Source

```bash
git clone https://github.com/SuperInstance/si-conservation-python.git
cd si-conservation-python

# Create a virtual environment
python -m venv .venv
source .venv/bin/activate

# Build and install with maturin
pip install maturin
maturin develop

# Verify
python -c "from si_conservation import Budget; print(Budget(100, 50))"
# Budget(gamma=100, eta=50, total=150)
```

### Prerequisites

- Python 3.8+
- Rust 1.70+ (for building from source)
- `maturin` Python package

---

## Quick Start

```python
from si_conservation import Budget, FleetBudget, ConservationGauge, check_budgets

# Single budget
b = Budget(gamma=143, eta=82)
print(f"Total: {b.total}")       # Total: 225
print(f"Valid: {b.check()}")     # Valid: True
print(repr(b))                   # Budget(gamma=143, eta=82, total=225)

# Fleet of budgets
fleet = FleetBudget([
    Budget(gamma=100, eta=50),
    Budget(gamma=43, eta=32),
])
print(fleet.total_gamma)         # 143
print(fleet.total_eta)           # 82
print(fleet.total)               # 225
print(fleet.invariant_holds())   # True
print(len(fleet))                # 2

# Conservation gauge — measure proportions
gauge = ConservationGauge(fleet)
print(f"γ fraction: {gauge.gamma_fraction():.4f}")   # 0.6356
print(f"η fraction: {gauge.eta_fraction():.4f}")      # 0.3644
print(f"Fractions valid: {gauge.fractions_valid()}")  # True

# Batch check
budgets = [Budget(100, 50), Budget(43, 32)]
print(check_budgets(budgets))    # True
```

---

## API Reference

### `Budget`

A single budget allocation with gamma (γ) and eta (η) components.

```python
class Budget:
    def __init__(self, gamma: int, eta: int) -> None: ...

    @property
    def gamma(self) -> int: ...     # Gamma (γ) component

    @property
    def eta(self) -> int: ...       # Eta (η) component

    @property
    def total(self) -> int: ...     # Total budget: gamma + eta

    def check(self) -> bool: ...    # Verify conservation law holds

    def __repr__(self) -> str: ...
    def __eq__(self, other: Budget) -> bool: ...
```

**Examples:**

```python
from si_conservation import Budget

# Create a budget
b = Budget(gamma=600, eta=400)
assert b.gamma == 600
assert b.eta == 400
assert b.total == 1000
assert b.check() is True

# Zero budget
z = Budget(gamma=0, eta=0)
assert z.total == 0
assert z.check() is True

# Equality comparison
a = Budget(gamma=100, eta=50)
c = Budget(gamma=100, eta=50)
assert a == c

# String representation
print(Budget(gamma=143, eta=82))
# Budget(gamma=143, eta=82, total=225)
```

### `FleetBudget`

A fleet-level budget aggregating multiple individual budgets.

```python
class FleetBudget:
    def __init__(self, budgets: list[Budget]) -> None: ...

    @property
    def total_gamma(self) -> int: ...    # Sum of all γ values

    @property
    def total_eta(self) -> int: ...      # Sum of all η values

    @property
    def total(self) -> int: ...          # Total fleet budget

    def invariant_holds(self) -> bool: ...  # Verify fleet-wide invariant

    @property
    def count(self) -> int: ...          # Number of budgets

    def __len__(self) -> int: ...
    def __repr__(self) -> str: ...
```

**Examples:**

```python
from si_conservation import Budget, FleetBudget

# Build a fleet from individual budgets
fleet = FleetBudget([
    Budget(gamma=100, eta=50),
    Budget(gamma=43, eta=32),
    Budget(gamma=200, eta=100),
])

print(fleet.total_gamma)         # 343
print(fleet.total_eta)           # 182
print(fleet.total)               # 525
print(fleet.invariant_holds())   # True
print(len(fleet))                # 3
print(fleet.count)               # 3
print(repr(fleet))
# FleetBudget(count=3, total_gamma=343, total_eta=182, total=525)

# Empty fleet
empty = FleetBudget([])
print(len(empty))                # 0
print(empty.total)               # 0
print(empty.invariant_holds())   # True
```

### `ConservationGauge`

Measures budget proportions across a fleet.

```python
class ConservationGauge:
    def __init__(self, fleet: FleetBudget) -> None: ...

    def gamma_fraction(self) -> float: ...  # Fraction of total in γ

    def eta_fraction(self) -> float: ...    # Fraction of total in η

    def fractions_valid(self) -> bool: ...  # γ_frac + η_frac ≈ 1.0

    def __repr__(self) -> str: ...
```

**Examples:**

```python
from si_conservation import Budget, FleetBudget, ConservationGauge

fleet = FleetBudget([
    Budget(gamma=100, eta=50),
    Budget(gamma=43, eta=32),
])

gauge = ConservationGauge(fleet)
print(f"γ fraction: {gauge.gamma_fraction():.6f}")   # 0.635556
print(f"η fraction: {gauge.eta_fraction():.6f}")      # 0.364444
print(f"Sum: {gauge.gamma_fraction() + gauge.eta_fraction():.6f}")  # 1.000000
print(f"Valid: {gauge.fractions_valid()}")             # True
print(repr(gauge))
# ConservationGauge(gamma=0.6356, eta=0.3644)

# Zero-budget fleet
empty_gauge = ConservationGauge(FleetBudget([]))
print(empty_gauge.gamma_fraction())  # 0.0
print(empty_gauge.eta_fraction())    # 0.0
```

### `check_budgets()`

Convenience function: check multiple budgets at once.

```python
def check_budgets(budgets: list[Budget]) -> bool: ...
```

Returns `True` if every budget satisfies the conservation law.

**Examples:**

```python
from si_conservation import Budget, check_budgets

# All valid
assert check_budgets([
    Budget(gamma=100, eta=50),
    Budget(gamma=43, eta=32),
]) is True

# Empty list
assert check_budgets([]) is True

# Mixed (all valid by construction)
assert check_budgets([
    Budget(gamma=0, eta=0),
    Budget(gamma=1000, eta=0),
    Budget(gamma=0, eta=1000),
]) is True
```

---

## Integration with si-cli

Use `si-conservation-python` alongside the `si-cli` Rust tool:

```python
#!/usr/bin/env python3
"""
Fleet budget analysis script.
Reads fleet.toml and verifies conservation using Python bindings.
"""

import subprocess
import json
from si_conservation import Budget, FleetBudget, ConservationGauge

def parse_fleet_toml(path: str) -> list[dict]:
    """Parse fleet.toml using Python's tomllib."""
    import tomllib
    with open(path, "rb") as f:
        data = tomllib.load(f)
    return data.get("agents", [])

def verify_fleet(fleet_path: str) -> None:
    """Verify conservation laws for a fleet config."""
    agents = parse_fleet_toml(fleet_path)
    budgets = [
        Budget(gamma=int(a["gamma"]), eta=int(a["h"]))
        for a in agents
    ]
    fleet = FleetBudget(budgets)
    gauge = ConservationGauge(fleet)

    print(f"Fleet: {len(budgets)} agents")
    print(f"  Total γ: {fleet.total_gamma}")
    print(f"  Total η: {fleet.total_eta}")
    print(f"  Total budget: {fleet.total}")
    print(f"  Invariant holds: {fleet.invariant_holds()}")
    print(f"  γ fraction: {gauge.gamma_fraction():.4f}")
    print(f"  η fraction: {gauge.eta_fraction():.4f}")

    if not fleet.invariant_holds():
        print("  ⚠ CONSERVATION VIOLATION DETECTED")
        # Cross-check with si-cli
        result = subprocess.run(
            ["si", "check", fleet_path],
            capture_output=True, text=True
        )
        print(result.stdout)

if __name__ == "__main__":
    verify_fleet("fleet.toml")
```

---

## Building and Publishing

### Development Build

```bash
# Install maturin
pip install maturin

# Build in development mode (editable install)
maturin develop

# Run Python tests
pip install pytest
pytest tests/
```

### Release Build

```bash
# Build a wheel
maturin build --release

# Build and publish to PyPI
maturin publish
```

### Cross-Platform Wheels

```bash
# Build for all supported platforms
maturin build --release --strip --manylinux off

# Using GitHub Actions (see .github/workflows/)
# Maturin-action handles cross-compilation automatically
```

---

## Architecture

```
src/
├── lib.rs            # PyO3 module definition, class registration
├── types.rs          # Python-facing types: PyBudget, PyFleetBudget, PyConservationGauge
└── conservation.rs   # Core Rust conservation logic (Budget, FleetBudget, ConservationGauge)
```

**How it works:**

1. `conservation.rs` defines the pure Rust types (`Budget`, `FleetBudget`, `ConservationGauge`) with all business logic
2. `types.rs` wraps them in PyO3 `#[pyclass]` types (`PyBudget`, `PyFleetBudget`, `PyConservationGauge`) that expose Python-friendly APIs
3. `lib.rs` registers all classes and creates Python aliases (`Budget` → `PyBudget`, etc.)

**Key Rust types:**

```rust
// conservation.rs
pub struct Budget {
    pub gamma: u64,
    pub eta: u64,
}

pub struct FleetBudget {
    pub budgets: Vec<Budget>,
}

pub struct ConservationGauge {
    fleet: FleetBudget,
}
```

**PyO3 wrapper pattern:**

```rust
// types.rs
#[pyclass]
pub struct PyBudget {
    inner: conservation::Budget,
}

#[pymethods]
impl PyBudget {
    #[new]
    #[pyo3(signature = (gamma, eta))]
    fn new(gamma: u64, eta: u64) -> Self { ... }

    #[getter]
    fn gamma(&self) -> u64 { ... }

    fn check(&self) -> bool { ... }
    fn __repr__(&self) -> String { ... }
    fn __eq__(&self, other: &Self) -> bool { ... }
}
```

---

## Performance

Since the core logic runs in Rust, the Python bindings have near-zero overhead:

```python
import time
from si_conservation import Budget, FleetBudget, check_budgets

# Create 100,000 budgets
start = time.perf_counter()
budgets = [Budget(gamma=i, eta=i * 2) for i in range(100_000)]
fleet = FleetBudget(budgets)
elapsed = time.perf_counter() - start
print(f"Created 100K budgets in {elapsed:.3f}s")

# Check all of them
start = time.perf_counter()
result = check_budgets(budgets)
elapsed = time.perf_counter() - start
print(f"Checked 100K budgets in {elapsed:.3f}s: {result}")
```

---

## Related Repos

| Repo | Language | Description |
|------|----------|-------------|
| [`conservation-law`](https://github.com/SuperInstance/conservation-law) | Rust | The core Rust crate this wraps |
| [`si-cli`](https://github.com/SuperInstance/si-cli) | Rust | CLI with `si check` for fleet.toml verification |
| [`si-fleet-api`](https://github.com/SuperInstance/si-fleet-api) | TypeScript | REST API for fleet budget management |
| [`si-runtime-python`](https://github.com/SuperInstance/si-runtime-python) | Python | Full Python runtime with conservation enforcement |
| [`si-runtime-go`](https://github.com/SuperInstance/si-runtime-go) | Go | Go runtime with conservation enforcement |

---

## License

MIT
