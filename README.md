# ArtCraftX Studio

A unified, sovereign creative suite launcher and workflow orchestrator built in pure Rust, powered by the **Martensite** GPU-accelerated retained-mode GUI engine.

![ArtCraftX Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode suite launcher, workspace switcher, and workflow orchestration UI.
- **`crates/engine`**: Inter-process RPC pipeline, shared GPU memory texture sharing, and cross-application project exchange.

## Features

- Zero-latency application switching across the entire ArtCraft creative ecosystem.
- Clean-room GPU retained-mode interface with zero external GUI dependencies.
- Deterministic, high-throughput IPC orchestration.

## Legal & Compliance Notice

ArtCraftX is an independent open-source creative suite shell. It is not affiliated with Adobe Inc. Adobe, Creative Cloud, Photoshop, and Illustrator are trademarks of Adobe Inc.

## License

ArtCraftX is dual-licensed under MIT or Apache-2.0.
