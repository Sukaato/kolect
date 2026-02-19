# Kolect


Kolect is a mobile-first application for organizing and tracking K-pop collection items (albums, lightsticks, photocards, merchandise, and related goods). The project combines a Vue 3 + TypeScript frontend with native packaging to deliver a fast, native-feeling experience on mobile devices.

## Purpose

- Allow collectors to catalog K-pop items (albums, lightsticks, photocards, merch) with images and metadata.
- Provide search, tagging, and simple inventory management to help collectors track what they own and what they still want.
- Target platforms: Android (primary target now); iOS support planned for a future release.

## Developer

- **Frontend:** Vue 3, TypeScript, Vite
- **Desktop shell:** Tauri (Rust)
- **Styling:** Tailwind CSS, DaisyUI, Sass
- **Icons:** lucide-vue-next
- **Tauri API:** @tauri-apps/api

### Prerequisites

- Node.js (recommended 18+)
- bun (or npm / yarn / pnpm)
- Rust toolchain (rustup + cargo) for Tauri native parts — follow the official Tauri setup for platform-specific dependencies: https://tauri.app/v1/guides/getting-started/prerequisites

### Install dependencies

Run from the repository root:

```bash
bun install
```

### Development

-- Run the full Tauri desktop app (starts frontend + Tauri dev server):

```bash
bun run dev
```

- Start the frontend only (Vite dev server) — useful for fast UI iteration:

```bash
bun run dev:front
```

If you run the frontend-only server and need to connect it to Tauri APIs for testing, run the full Tauri dev flow (`bun run dev`) instead.

### Build


- Build the frontend only:

```bash
bun run build:front
```

- Build native bundles for distribution (Tauri build):

```bash
bun run build
```

## Recommended IDE setup

- VS Code with the following extensions: Vue (Volar), Tauri, rust-analyzer

## Contributing

Thanks for considering contributing to Kolect! A few quick guidelines to get started:

- **Issues:** Open an issue to discuss large changes or propose features before implementing.
- **Branches:** Create a feature branch from `main` named `feat/your-feature` or `fix/your-bug`.
- **Commits & PRs:** Keep commits focused and open a PR with a clear description. Link related issues.
- **Formatting:** Follow existing code style. Run the dev frontend locally to verify UI changes.
- **Native builds:** If you change Tauri/native code, ensure the Rust toolchain is installed and native builds still pass on your platform.

Maintainers will review PRs and request changes as needed. If you'd like, I can add a CONTRIBUTING.md with a checklist and PR template.
---

If you'd like, I can also add a short CONTRIBUTING section, example data, or a quick start walkthrough for adding items to a collection. Which would you prefer next?
