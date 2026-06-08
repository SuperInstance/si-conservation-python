# si-conservation-python

> Python bindings for the **SuperInstance conservation law** — `γ + η = total_budget` via PyO3.

[![Crates.io](https://img.shields.io/crates/v/si-conservation.svg)](https://crates.io/crates/si-conservation)
[![PyPI](https://img.shields.io/pypi/v/si-conservation.svg)](https://pypi.org/project/si-conservation/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build](https://github.com/SuperInstance/si-conservation-python/actions/workflows/ci.yml/badge.svg)](https://github.com/SuperInstance/si-conservation-python/actions/workflows/ci.yml)

## Overview

The **SuperInstance conservation law** is a fundamental invariant governing budget allocation across
all nodes in a SuperInstance fleet. At every level — individual nodes, fleet aggregates, and globally —
the following must hold:

```
γ (gamma) + η (eta) = total_budget
```

This crate provides **Python bindings** to the core Rust implementation, enabling Python developers
to create, inspect, and verify budget allocations with native performance through PyO3.

## Features

- **`Budget`** — Individual node budget with gamma/eta components
- **`FleetBudget`** — Aggregated fleet-level budget across multiple nodes
- **`ConservationGauge`** — Proportional analysis of budget allocation
- **`check_budgets()`** — Batch verification helper
- Zero-cost abstractions backed by Rust
- Full type hints via `pyi` stubs
- Compatible with `si-cli` for fleet management workflows

## Installation

### From PyPI (recommended)

```bash
pip install si-conservation
```

### From source with maturin

```bash
# Clone the repository
git clone https://github.com/SuperInstance/si-conservation-python.git
cd si-conservation-python

# Create a virtual environment
python -m venv .venv
source .venv/bin/activate

# Install maturin
pip install maturin

# Build and install in development mode
maturin develop

# Or build a wheel
maturin build --release
```

### Requirements

- Python ≥ 3.8
- A Rust toolchain (`rustup` recommended)
- `maturin` ≥ 1.0

## Quick Start

```python
import si_conservation

# Create a single budget allocation
b = si_conservation.Budget(gamma=143, eta=82)
print(b)  # Budget(gamma=143, eta=82, total=225)

# Access properties
assert b.gamma == 143
assert b.eta == 82
assert b.total == 225

# Verify the conservation law
assert b.check()  # gamma + eta == total ✓
```

## Usage

### Individual Budgets

```python
import si_conservation

# Basic budget creation
b = si_conservation.Budget(gamma=100, eta=50)

# Properties are computed from the Rust core
print(f"Gamma: {b.gamma}")    # 100
print(f"Eta: {b.eta}")        # 50
print(f"Total: {b.total}")    # 150

# Conservation check
if b.check():
    print("✓ Budget is valid")
else:
    print("✗ Conservation law violated!")

# Equality comparison
b1 = si_conservation.Budget(gamma=100, eta=50)
b2 = si_conservation.Budget(gamma=100, eta=50)
assert b1 == b2

# String representation
print(repr(b1))  # Budget(gamma=100, eta=50, total=150)
```

### Fleet-Level Budgets

```python
import si_conservation

# Create a fleet from multiple budgets
fleet = si_conservation.FleetBudget([
    si_conservation.Budget(gamma=100, eta=50),
    si_conservation.Budget(gamma=43, eta=32),
])

# Aggregate properties
print(fleet.total_gamma)  # 143
print(fleet.total_eta)    # 82
print(fleet.total)        # 225
print(fleet.count)        # 2

# Verify fleet-level invariant
assert fleet.invariant_holds()
print(fleet)  # FleetBudget(count=2, total_gamma=143, total_eta=82, total=225)

# Use len()
assert len(fleet) == 2
```

### Conservation Gauge

The `ConservationGauge` provides fractional analysis of budget allocation:

```python
import si_conservation

fleet = si_conservation.FleetBudget([
    si_conservation.Budget(gamma=143, eta=82),
])

gauge = si_conservation.ConservationGauge(fleet)

# Fraction of total allocated to gamma vs eta
gamma_frac = gauge.gamma_fraction()  # ~0.6356
eta_frac = gauge.eta_fraction()      # ~0.3644

print(f"Gamma fraction: {gamma_frac:.4f}")  # 0.6356
print(f"Eta fraction: {eta_frac:.4f}")      # 0.3644

# Verify fractions sum to 1.0
assert gauge.fractions_valid()

print(gauge)  # ConservationGauge(gamma=0.6356, eta=0.3644)
```

### Batch Verification

```python
import si_conservation

budgets = [
    si_conservation.Budget(gamma=100, eta=50),
    si_conservation.Budget(gamma=43, eta=32),
    si_conservation.Budget(gamma=200, eta=100),
]

# Check all budgets at once
if si_conservation.check_budgets(budgets):
    print("All budgets satisfy the conservation law ✓")
```

### Integration with si-cli

The `si-conservation` library integrates with the `si-cli` tool for fleet management:

```python
import si_conservation
import subprocess
import json

# Example: fetch fleet status from si-cli
# result = subprocess.run(["si-cli", "fleet", "status", "--json"], capture_output=True, text=True)
# fleet_data = json.loads(result.stdout)

# Build budgets from fleet data (example)
budgets = []
# for node in fleet_data["nodes"]:
#     budgets.append(si_conservation.Budget(
#         gamma=node["gamma"],
#         eta=node["eta"],
#     ))

# For demonstration:
budgets = [
    si_conservation.Budget(gamma=100, eta=50),
    si_conservation.Budget(gamma=43, eta=32),
]

fleet = si_conservation.FleetBudget(budgets)

# Validate before deploying
assert fleet.invariant_holds(), "Fleet budget invariant violated!"

# Get gauge readings for monitoring
gauge = si_conservation.ConservationGauge(fleet)
print(f"Gamma allocation: {gauge.gamma_fraction():.2%}")
print(f"Eta allocation: {gauge.eta_fraction():.2%}")

# Export metrics for monitoring systems
metrics = {
    "total_gamma": fleet.total_gamma,
    "total_eta": fleet.total_eta,
    "total_budget": fleet.total,
    "node_count": fleet.count,
    "gamma_fraction": gauge.gamma_fraction(),
    "eta_fraction": gauge.eta_fraction(),
    "invariant_holds": fleet.invariant_holds(),
}
print(f"Metrics: {json.dumps(metrics, indent=2)}")
```

## API Reference

### `Budget(gamma, eta)`

Create a budget allocation with the specified gamma and eta values.

| Property  | Type  | Description                    |
|-----------|-------|--------------------------------|
| `gamma`   | `int` | Gamma (γ) component            |
| `eta`     | `int` | Eta (η) component              |
| `total`   | `int` | Total budget (gamma + eta)     |

| Method    | Return Type | Description                          |
|-----------|-------------|--------------------------------------|
| `check()` | `bool`      | Verify γ + η == total                |

### `FleetBudget(budgets)`

Create a fleet-level budget from a list of `Budget` objects.

| Property       | Type  | Description                        |
|----------------|-------|------------------------------------|
| `total_gamma`  | `int` | Sum of all gamma values            |
| `total_eta`    | `int` | Sum of all eta values              |
| `total`        | `int` | Total fleet budget                 |
| `count`        | `int` | Number of budgets in the fleet     |

| Method              | Return Type | Description                          |
|---------------------|-------------|--------------------------------------|
| `invariant_holds()` | `bool`      | Verify fleet-level conservation law  |

Supports `len()` and `repr()`.

### `ConservationGauge(fleet)`

Create a gauge for proportional analysis of a fleet's budget.

| Method               | Return Type | Description                              |
|----------------------|-------------|------------------------------------------|
| `gamma_fraction()`   | `float`     | Fraction of total allocated to gamma     |
| `eta_fraction()`     | `float`     | Fraction of total allocated to eta       |
| `fractions_valid()`  | `bool`      | Verify fractions sum to 1.0              |

### `check_budgets(budgets)`

Batch-verify a list of `Budget` objects. Returns `True` if all budgets satisfy
the conservation law.

## Architecture

```
┌─────────────────────────────────────┐
│         Python (CPython)            │
│                                     │
│  import si_conservation             │
│  b = Budget(gamma=143, eta=82)      │
└──────────────┬──────────────────────┘
               │ PyO3 FFI
               │
┌──────────────▼──────────────────────┐
│         Rust (si_conservation)       │
│                                     │
│  conservation::Budget { gamma, eta } │
│  conservation::FleetBudget { ... }   │
│  conservation::ConservationGauge     │
└─────────────────────────────────────┘
```

The Rust core implements the conservation law logic with zero-cost abstractions.
PyO3 bindings expose this to Python with minimal overhead — no serialization,
no intermediate representations, just direct FFI calls.

## Development

### Setup

```bash
# Clone
git clone https://github.com/SuperInstance/si-conservation-python.git
cd si-conservation-python

# Create venv
python -m venv .venv
source .venv/bin/activate

# Install dev dependencies
pip install maturin pytest

# Build in dev mode (fast incremental builds)
maturin develop
```

### Running Tests

```bash
# Run all Python tests
pytest tests/ -v

# Run with coverage
pytest tests/ -v --cov=si_conservation

# Run Rust tests
cargo test
```

### Building for Release

```bash
# Build wheel
maturin build --release

# The wheel will be in target/wheels/
pip install target/wheels/si_conservation-*.whl
```

### Publishing

```bash
# Publish to PyPI
maturin publish

# Publish to TestPyPI first
maturin publish --repository testpypi
```

## Performance

Because the core logic runs in Rust with PyO3 bindings, operations are significantly
faster than pure-Python equivalents:

```python
import si_conservation
import time

# Create 100,000 budgets
budgets = [si_conservation.Budget(gamma=i, eta=i*2) for i in range(100_000)]

# Fleet aggregation
start = time.perf_counter()
fleet = si_conservation.FleetBudget(budgets)
elapsed = time.perf_counter() - start
print(f"Fleet creation: {elapsed*1000:.2f}ms")

# Gauge computation
start = time.perf_counter()
gauge = si_conservation.ConservationGauge(fleet)
frac = gauge.gamma_fraction()
elapsed = time.perf_counter() - start
print(f"Gauge computation: {elapsed*1000:.4f}ms")
```

Typical results show sub-millisecond operations even for large fleets, thanks to
Rust's zero-cost abstractions and PyO3's efficient FFI.

## Error Handling

```python
import si_conservation

# Negative values are rejected by type (u64)
# Budget(gamma=-1, eta=0)  # TypeError: can't convert negative int to unsigned

# Zero budgets are valid
b = si_conservation.Budget(gamma=0, eta=0)
assert b.total == 0
assert b.check()

# Empty fleets are valid
fleet = si_conservation.FleetBudget([])
assert fleet.total == 0
assert fleet.total_gamma == 0
assert fleet.total_eta == 0
assert fleet.invariant_holds()

# Gauge on empty fleet returns 0.0
gauge = si_conservation.ConservationGauge(fleet)
assert gauge.gamma_fraction() == 0.0
assert gauge.eta_fraction() == 0.0
```

## Type Hints

The library ships with type hints for IDE autocompletion:

```python
from si_conservation import Budget, FleetBudget, ConservationGauge, check_budgets
from typing import List

def validate_fleet(budgets: List[Budget]) -> bool:
    """Validate a fleet of budgets."""
    if not check_budgets(budgets):
        return False
    fleet = FleetBudget(budgets)
    return fleet.invariant_holds()

def allocation_ratio(fleet: FleetBudget) -> float:
    """Get the gamma/eta allocation ratio."""
    gauge = ConservationGauge(fleet)
    return gauge.gamma_fraction()
```

## Related Projects

- **[si-conservation](https://github.com/SuperInstance/si-conservation)** — Core Rust crate for the conservation law
- **[si-cli](https://github.com/SuperInstance/si-cli)** — CLI tool for fleet management and budget inspection
- **[si-conservation-js](https://github.com/SuperInstance/si-conservation-js)** — JavaScript/WASM bindings

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`pytest tests/ -v && cargo test`)
5. Commit (`git commit -m 'Add amazing feature'`)
6. Push (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

MIT License. See [LICENSE](LICENSE) for details.
