# Electron Cloud (OpenGL Edition)

A hydrogenic orbital visualization toolkit written in Rust using OpenGL (GL + GLSL) through C library bindings.

This version focuses on lower-level GPU control and explicit shader-based rendering while preserving the same quantum mechanical sampling framework as the wgpu edition.

---

## Overview

Electron Cloud (OpenGL Edition) generates Monte Carlo samples of hydrogenic orbitals defined by quantum numbers (n, l, m) and renders them using OpenGL instanced drawing.

Instead of plotting analytic surfaces, the wavefunction is sampled probabilistically:

- Radial distribution sampled from |Rₙₗ(r)|² r²  
- Angular distribution sampled from |Pₗᵐ(cosθ)|² sinθ  
- Azimuthal angle φ sampled uniformly  

Each sample is rendered as a small sphere instance.  
Color encodes probability density intensity.

This implementation uses:

- OpenGL for rendering
- GLSL vertex and fragment shaders
- Explicit buffer management
- C-library bindings via Rust FFI

---

## Why an OpenGL Version?

The wgpu version abstracts much of the graphics pipeline.

This version intentionally moves closer to the metal:

- Manual shader compilation (GLSL)
- Explicit VAO/VBO setup
- Direct buffer uploads
- Classic rendering pipeline control

It exists to better understand:

- How instanced rendering works in traditional OpenGL
- How GPU memory buffers are structured
- How vertex attributes are passed to shaders
- The differences between modern Rust-native GPU APIs and C-based graphics APIs

---

## Architecture

### Physics Layer

Identical to the Rust-native version:

- Associated Laguerre polynomials
- Associated Legendre polynomials
- Radial and angular CDF construction
- Inverse transform sampling
- Monte Carlo particle generation

All numerical sampling is CPU-side in `f64`.

---

### Rendering Layer (OpenGL)

- Sphere mesh generated on CPU
- Vertex Buffer Objects (VBO)
- Vertex Array Objects (VAO)
- Instanced attributes for position and color
- GLSL shaders for transformation and color output
- Depth testing enabled

The vertex shader handles:

- Model scaling
- Instance translation
- View-projection transformation

The fragment shader outputs color directly.

---

## Numerical Strategy

### Radial Sampling

- Discretized CDF
- Cached per (n, l)
- Binary search inversion

### Angular Sampling

- Discretized CDF
- Cached per (l, |m|)

This avoids recomputing expensive polynomial evaluations for every particle.

---

## Controls

Mouse Drag → Orbit camera  
Scroll → Zoom  
Escape → Exit  

---

## Running

Ensure OpenGL support is available on your system.

```
cargo run --release
```

You will be prompted for:

- Principal quantum number (n)
- Azimuthal quantum number (l)
- Magnetic quantum number (m)
- Particle count

---

## Differences from the wgpu Version

| wgpu Version           | OpenGL Version               |
| ---------------------- | ---------------------------- |
| Rust-native GPU API    | C-based OpenGL bindings      |
| Pipeline descriptors   | Manual shader + buffer setup |
| Implicit safety layers | Explicit state management    |
| Modern abstraction     | Classic graphics pipeline    |

The physics is identical.  
The rendering philosophy is different.

---

## Limitations

- Hydrogenic orbitals only
- CPU-based sampling
- No lighting model
- Performance bound by particle count
- Requires OpenGL support

---

## Design Philosophy

This edition prioritizes understanding traditional GPU pipelines.

Where the wgpu version explores modern Rust GPU abstractions, this version explores:

- Graphics pipeline fundamentals
- Shader programming
- Manual buffer orchestration
- The cost of explicit state control

Both versions serve as computational learning tools rather than production quantum chemistry software.

---

## Author

Lnifelias Stargarden  
(Real name: Bhaskar Malviya)  

Computational Chemistry | Quantum Chemistry | Scientific Programming  
