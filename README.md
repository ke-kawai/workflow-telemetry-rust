# Workflow Telemetry Rust

CPU monitoring for GitHub Actions. Outputs to `$GITHUB_STEP_SUMMARY`.

## Usage

```yaml
- uses: ke-kawai/workflow-telemetry-action/workflow-telemetry-rust@main
  with:
    metric_frequency: "5"
    metric_iterations: "60"
```

## Current Status: MVP

Phase 1 complete. CPU monitoring only. See `docs/DESIGN.md` for full roadmap.
