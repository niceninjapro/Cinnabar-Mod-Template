# Cinnabar mod template

A Rust/WebAssembly personal mod built with Cinnabar's `mod-api`. The starter publishes a HUD label and changes it when the demo key is pressed; it requests no permissions.

## Build

Install the Rust toolchain and `wasm32-unknown-unknown` target pinned in `rust-toolchain.toml`, then run:

```powershell
python scripts\build-mod.py
cargo test --locked
```

The importable component is written to `dist\mod-template.wasm`. Import it in Cinnabar through **Settings → Mods → Import**.

## Developing with AI

Before implementing a feature, consult the [Cinnabar modding guide](https://cinnabar.gg/docs/modding/), its relevant guide and API reference, and a matching example in the sibling `cinnabar\examples\mods` checkout. The `mod-api` dependency is pinned to a Cinnabar source revision in the workspace `Cargo.toml`; its generated bindings are the source of truth for available interfaces.

Describe the intended player-facing behavior, target world, relevant inputs or game state, restrictions, and how to verify it. Ask the assistant to inspect the docs and examples, identify the minimum grants, implement the feature, and run the build and tests. Do not rely on guessed API names or add OS, filesystem, network, or WASI access: personal mods run in a sandbox and communicate through documented capabilities.

Personal mods do not have direct world-editing or raw-packet access. If the feature requires a server guest or trusted packet access, choose the corresponding mod type and follow its separate guide instead of extending this personal-mod template.

## Project conventions

You should:

- Keep the Rust/WASM component entry point and Cinnabar packaging working.
- Add only the permissions the feature actually needs in `mods\template\pack\grants.json`.
- Implement lifecycle and per-frame behavior in `mods\template\src\guest.rs`.
- Replace `assets\base-icon.png` to change the icon shown on the mod's card in Cinnabar's Mods UI. It is embedded when building the component; keep it a PNG no larger than 256 × 256 pixels and 256 KiB.
- Add dependencies only when necessary and keep the build command working.
- Check returned errors from host API calls and keep frame work within the documented fuel budget.
- Explain the required permissions and how to build/import the finished mod.

`init` runs when the mod is initialized, and `frame` runs on admitted frames. Use the Cinnabar API through `mod-api`; do not add operating-system or WASI imports.

## Template files

- `mods\template\src\guest.rs` — mod lifecycle and behavior.
- `mods\template\pack\grants.json` — host capabilities/permissions.
- `mods\template\pack\src\main.rs` — converts the compiled module into a validated Cinnabar component.
- `assets\base-icon.png` — base mod-card icon, the Cinnabar logo embedded by the packer.
- `scripts\build-mod.py` — builds the component.
