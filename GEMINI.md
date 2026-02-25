# Gemini Project: Atom Simulator

## Project Overview

This project is a high-quality, idiomatic Rust rewrite of a C++ application that visualizes the probabilistic electron cloud of a hydrogen atom. It uses OpenGL for 3D rendering and provides an interactive camera for inspecting the generated atomic orbitals.

The simulation's core is a Rust translation of complex physics formulas that sample the Schrödinger wave function for a given set of quantum numbers (`n`, `l`, `m`) to generate a particle cloud. The resulting visualization accurately represents the intricate shapes of atomic orbitals.

The project is architected with modern Rust principles, including modular design, RAII for resource management, and safe abstractions over the `unsafe` OpenGL API.

## Building and Running

The project is built and run using the standard Rust toolchain (Cargo).

1.  **Navigate to the project root**:
    ```sh
    cd /home/lnife/AI-Workspace/Atom_Simulator
    ```
2.  **Build and Run**:
    ```sh
    cargo run
    ```
    -   On the first run, Cargo will download and compile all dependencies.
    -   The program will then prompt you in the terminal to enter the initial quantum numbers. You can enter an integer or press Enter to accept the suggested defaults.
    -   After input, the graphics window will launch, displaying the generated particle cloud.

## How to Use the Application

-   **Initial State**: Define the orbital to visualize by entering the **principal (n)**, **azimuthal (l)**, and **magnetic (m)** quantum numbers in the terminal on startup.
-   **Camera Controls**:
    -   **Orbit**: Click and drag the left mouse button.
    -   **Zoom**: Use the mouse's scroll wheel.
-   **Exit**: Press the `Escape` key or close the graphics window.

## Development Conventions

-   **Modularity**: The codebase is organized into logical modules found in `src/`:
    -   `main.rs`: Application entry point, windowing, and the main render loop.
    -   `render.rs`: A rendering engine providing safe abstractions (`ShaderProgram`, `VertexArray`) over the OpenGL API.
    -   `camera.rs`: Manages the 3D camera, view matrix, and mouse-based interaction.
    -   `physics.rs`: The core simulation engine. Contains all logic for particle generation based on quantum mechanics.

-   **Resource Management (RAII)**: The `render` module makes extensive use of the `Drop` trait. `ShaderProgram` and `VertexArray` structs automatically deallocate their corresponding GPU resources when they go out of scope, preventing resource leaks.

-   **State Management**:
    -   Global simulation state (the quantum numbers `n`, `l`, `m`) is managed in `physics.rs` behind a `Mutex` for safe access.
    -   UI state (like the camera) that needs to be shared between the main loop and event handlers is wrapped in `Arc<Mutex<>>` for safe, shared mutability.

-   **Performance**: Expensive, one-time calculations in the physics engine (such as generating Cumulative Distribution Function tables) are handled by the `lazy_static` crate to ensure they are computed only once per run.
