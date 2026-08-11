# Parameter Spaces

A study's inputs are named, bounded intervals. `Parameter<'a, T>` carries a
`Cow<'a, str>` identity and a `[lower, upper)` sampling interval, and the
compile-time-dimensional `ParameterSpace<'a, T, PARAMETERS>` composes an
array of them into one validated space.

## Validation

`Parameter::borrowed` (or `Parameter::owned` for a heap-held name) rejects an
empty name (`InvalidParameter::EmptyName`), non-finite bounds
(`InvalidParameter::NonFiniteBounds`), and `lower >= upper`
(`InvalidParameter::UnorderedBounds`). `ParameterSpace::new` additionally
rejects zero dimensions (`SpaceError::ZeroDimensions`) and duplicate names
(`SpaceError::DuplicateName`), so every coordinate of a study is addressable
by a stable label.

## Mapping

Designs produce unit-hypercube points. `Parameter::map_unit` scales one
normalized coordinate into its parameter interval, and
`ParameterSpace::map_unit_into` maps a full `[f64; PARAMETERS]` unit point
into caller-provided `[T; PARAMETERS]` storage without allocating. The
`study::Study`/`Sample` seam pairs such a mapped point with its responses
before persistence through the Consus adapter.

The following is a focused, non-standalone API fragment:

```rust,ignore
use tyche_core::design::{Parameter, ParameterSpace};

let temperature = Parameter::borrowed("temperature", 300.0, 800.0)?;
let pressure = Parameter::borrowed("pressure", 1.0e5, 5.0e5)?;
let space = ParameterSpace::new([temperature, pressure])?;
let mut physical = [0.0; 2];
space.map_unit_into(&[0.5, 0.25], &mut physical);
```

See the [LHC sampling example](examples/lhc_sampling.md) for the full sweep:
a `LatinHypercube` design feeds normalized points through the same mapping.
