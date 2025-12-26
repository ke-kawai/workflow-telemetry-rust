# Workflow Telemetry

Monitor CPU and memory usage during GitHub Actions workflow execution with automatic charts generation.

## Features

- 🚀 **Easy to use** - Just add one line to your workflow
- 📊 **Automatic charts** - SVG charts generated and displayed in workflow summary
- 🎯 **Dual-axis visualization** - CPU and memory in a single combined chart
- 🔄 **Automatic cleanup** - No manual process management required
- ⚡ **Lightweight** - Rust-based monitoring with minimal overhead

## Usage

Add this action at the beginning of your job:

```yaml
steps:
  - uses: ke-kawai/workflow-telemetry-rust@v1
    with:
      interval: '2'  # Data collection interval in seconds (optional, default: 2)

  - name: Your build steps
    run: npm test

  # Charts are automatically generated and displayed in the workflow summary
```

## Example

```yaml
name: CI with Telemetry

on: [push]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ke-kawai/workflow-telemetry-rust@v1

      - name: Run tests
        run: npm test

      # Telemetry automatically stops and generates charts at job end
```

## Output

The action automatically generates:
- **Combined chart** (CPU + Memory) in workflow summary
- **Statistics** including average and peak usage
- **Individual charts** available in collapsible section

## How It Works

1. **Start**: The action starts monitoring in the background when called
2. **Monitor**: Collects CPU and memory data at specified intervals
3. **Finish**: Automatically stops monitoring and generates charts when the job completes
4. **Display**: Shows results in GitHub Actions step summary

## Technical Details

- Built with Rust for performance
- Uses GitHub Actions' post-job hooks for automatic cleanup
- Generates SVG charts using charts-rs
- No manual process management required

## License

MIT
