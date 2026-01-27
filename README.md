# Reproductions of Micro Projects

## 📚 Introduction
This repository contains reproductions of various micro projects aimed at **hands-on learning** and **practical skill development**.

## 🎯 Purpose
* **Practice & Consolidation**: Strengthen understanding of language fundamentals
  * Standard library usage
  * Core language features
  * Best practices and patterns
* **Language Exploration**: Experiment with new programming languages
* **Skill Expansion**: Learn and apply new techniques through practical implementation

## 📁 Projects

### 🎮 SIMPLE3D - Basic 3D Cube Renderer
* **Date**: 2026/01/23
* **Languages**: HTML, JavaScript
* **Tools**: Firefox
* **Description**: A **basic 3D graphics rendering engine** implemented with pure JavaScript and HTML5 Canvas, demonstrating the complete rendering pipeline from 3D space to 2D screen
* **Features**:
  - **Wireframe Rendering**: Display cube edges and vertices
  - **Perspective Projection**: Simulates 3D effects as seen by the human eye
  - **Dynamic Rotation**: Supports continuous Y-axis rotation animation
  - **Coordinate Transformation**: Implements a complete 3D graphics rendering pipeline
* **Learning Focus**:
  - 3D coordinate systems and transformation matrices
  - Model-View-Projection transformation chain
  - Simple animation loop implementation
  - Canvas 2D drawing API usage
* **Key Implementation**:
  - Perspective projection (`z` coordinate division)
  - 3D rotation around Y-axis
  - Coordinate mapping from normalized [-1, 1] to screen coordinates
  - Dual animation loop support (both `setTimeout` and `requestAnimationFrame`)

### 🎨 PPMDEMO - Procedural Shader Animation
* **Date**: 2026/01/25
* **Languages**: C, C++, Rust
* **Tools**: FFmpeg, MPV
* **Description**: A procedural shader animation rendered to PPM image sequences, exploring:
  * GPU shader algorithms on CPU
  * Multi-threaded rendering optimization
  * Cross-language implementation comparison
  * Media encoding pipeline
* **Learning Journey**:
  - Technical migration from GLSL shaders to CPU implementation
  - Rust multi-threading performance optimization practice
  - Cross-language algorithm implementation comparison
  - Image sequence to video encoding processing
* **Key Features**:
  - Fractal/procedural texture generation
  - Mathematical visualization of vector operations
  - Multi-threaded frame rendering
  - PPM image format output

## 🚀 Getting Started
Each project contains its own documentation and build instructions. Clone the repository and explore individual project directories for specific details.

## 📈 Learning Journey
These projects represent incremental steps in understanding:
- **Graphics Programming**: From basic 3D concepts to shader algorithms
- **Performance Optimization**: Single-threaded to multi-threaded implementations
- **Language Comparison**: Solving the same problem across different programming paradigms
- **Tool Integration**: Using specialized tools (FFmpeg, MPV) for media processing
- **Mathematical Visualization**: Translating mathematical formulas into visual effects

---

*"The best way to learn is by doing. These projects are my hands-on laboratory for exploration and growth."*