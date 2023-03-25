# Dzahui

Dzahui is a library oriented to real-time simulation of ODE/PDE via Finite Element Method and OpenGL providing a simple interface.

To use it, one simply creates an *.obj* file (see [blender](https://www.blender.org/) and [obj files](https://en.wikipedia.org/wiki/Wavefront_.obj_file)) composed of triangles representing a mesh, specifies initial and/or boundary conditions:
```rust
let naviers_params = StokesParams::static_pressure().
    hydrostatic_pressure(100_f64)
    .density(1_f64)
    .force_function(
        Box::new(|_| -10_f64)
    )
    .build();
```
builds a window indicating the problem to solve:

```rust
let window_builder: DzahuiWindowBuilder = DzahuiWindow::builder("./assets/1dbar.obj")
    .solve_static_pressure(naviers_params)
    .with_integration_iteration(350);

let window = window_builder.build();
```

and runs the window:
```rust
window.run();
```

The resulting mesh will be colored from blue to red indicating
speed (or pressure in the case of the hydrostatic pressure equation).

## Available equations

* 1D time-dependent diffussion equation
* 1D time-independent diffussion equation
* Hysrostatic pressure equation (Or simplified 1D time-independent Navier-Stokes for newtonian fluids)

For now, only three equations with Dirichlet boundaries are implemented, but more will be added in the future, including two an three-dimensional cases of Navier Stokes.

## How to use
Dzahui is available on [crates.io](https://crates.io/crates/Dzahui). Only add it as a dependency and follow one of the many binaries available in the crate to generate a simulation.

## Future implementations

* Simmulate various types of curves akin to the problem being solved (like streamlines, pathlines and streaklines).
* Improve GUI by adding buttons and graphs.
* Improve method to create boundary conditions via GUI and
 function-defined boundaries.
* Include Newmann conditions.
* Implement 2D and 3D integtration algorithms.
* Correct some implementations to improve performance.
* Increase polynomial degree to be used on problems.
* Use macros to embed newly user-created equations into the
possible equations to simmulate.

Dzahui is a project to **mayor in physics** at Facultad de Ciencias, UNAM.

## License
[MIT](https://mit-license.org/)