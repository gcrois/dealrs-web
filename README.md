# dealrs-web

Minimal Dioxus + Tailwind site demonstrating the [dealrs](https://crates.io/crates/dealrs) crate.

## Requirements

- Rust (rustup / cargo)
- pnpm (preferred; used to install Tailwind CLI)
- Dioxus CLI (`dx`)
- just (to run tasks from the included Justfile)
- Git (to clone repository)

Ensure `$HOME/.cargo/bin` is in your PATH so `cargo`-installed tools (like `dx`) are available.

## Install dependencies

1. Install [Rust](https://www.rust-lang.org/tools/install)

2. Install [dioxus-cli](https://dioxuslabs.com/learn/0.6/getting_started/) (provides `dx`)

3. Install [pnpm](https://pnpm.io/installation)

4. Install [just](https://github.com/casey/just) (if you want to use the provided Justfile)

5. Install Node dependencies (Tailwind CLI)
   - pnpm install

## Development (generate styles + hot reload)

- Terminal 1: generate/watch Tailwind CSS:
  `just serve-style`


- Terminal 2: run Dioxus dev server (hot reload):
  `just serve`

Tailwind source is `tailwind.css`; generated CSS is written to `assets/GENERATED_tailwind.css`.

## Build (production)

- `just build`
