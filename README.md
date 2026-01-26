# Nassau-WASM

Nautical performance calculator + metrics, written in Rust. Running in the browser via WASM. "For the Plot" + Educational use ONLY.

## Tech Stack

- Rust
- Next.js (Typescript)
- WASM

### Goal

Given wind + boat inputs, will compute sail trim quality and estimated speed efficiency.

**Use inputs:**

```bash
- User inputs (Next.js UI)
- True wind speed (knots)
- True wind angle (0–180° relative to bow)
- Boat heading (degrees)
- Sail angle (degrees off centerline)
```

**Backend Computes:**

- **Apparent wind angle:** Wind felt on the boat after accounting for heading
- **Optimal sail angle:** Simple rule-of-thumb curve (not CFD)
- **Trim score (0–100):** How close current sail angle is to optimal
- **Speed factor (0.0–1.0)**: Rough efficiency multiplier based on angle + trim

### App Output:

- Numeric readouts
- A simple gauge / bar
- A short label:
    - “Close-hauled”
    - “Beam reach”
    - “Broad reach”
    - “Running”

### License

MIT