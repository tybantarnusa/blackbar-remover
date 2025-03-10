# Blackbar Remover

## Introduction

Blackbar Remover is a simple and efficient tool designed to remove black bars from images quickly. This web application allows users to select an image and automatically removes any black bars present, providing a clean and adjusted image.

## Getting Started

### Prerequisites

- Rust
- Node

### Building

1. Run wasm-pack:
   ```bash
   wasm-pack build --target web --out-dir ./web/pkg
   ```

### Running Web Locally

1. Get inside web:
   ```bash
   cd web/
   ```
2. Run vite development server:
   ```bash
   npm run dev
   ```
