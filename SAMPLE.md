# Sample Output

This document shows example output from the workflow telemetry action.

## CPU Usage

```mermaid
xychart-beta
    title "CPU Usage"
    x-axis ["0s", "5s", "10s", "15s", "20s"]
    y-axis "CPU %" 0 --> 100
    line [12.5, 18.3, 15.8, 22.1, 16.4]
```

## Memory Usage

```mermaid
xychart-beta
    title "Memory Usage"
    x-axis ["0s", "5s", "10s", "15s", "20s"]
    y-axis "Memory GB" 0 --> 16.0
    line [4.52, 4.78, 5.21, 5.43, 5.38]
```

## Features

- **Separate charts**: CPU and Memory are shown independently with appropriate scales
- **CPU scale**: Fixed 0-100% for easy comparison
- **Memory scale**: 0 to total system memory (GB)
- **Line charts**: Shows trend over time clearly
- **Relative time**: X-axis shows elapsed time from start (0s, 5s, 10s...)
