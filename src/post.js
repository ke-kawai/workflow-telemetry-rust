const core = require('@actions/core');
const exec = require('@actions/exec');
const fs = require('fs');
const path = require('path');

async function post() {
  try {
    core.info('Finishing telemetry monitoring...');

    // Get PID from state
    const pid = core.getState('telemetry-pid');
    const pidFile = '/tmp/telemetry.pid';

    if (!pid && !fs.existsSync(pidFile)) {
      core.warning('Telemetry PID not found');
      return;
    }

    const telemetryPid = pid || fs.readFileSync(pidFile, 'utf8').trim();
    core.info(`Stopping telemetry (PID: ${telemetryPid})...`);

    // Send SIGTERM to gracefully stop the process
    try {
      process.kill(telemetryPid, 'SIGTERM');
    } catch (error) {
      core.warning(`Failed to send SIGTERM: ${error.message}`);
    }

    // Wait for process to finish (max 30 seconds)
    for (let i = 0; i < 30; i++) {
      try {
        process.kill(telemetryPid, 0); // Check if process exists
        await new Promise(resolve => setTimeout(resolve, 1000));
      } catch {
        core.info('Telemetry process stopped successfully');
        break;
      }
    }

    // Force kill if still running
    try {
      process.kill(telemetryPid, 0);
      core.warning('Force killing telemetry process...');
      process.kill(telemetryPid, 'SIGKILL');
    } catch {
      // Process already stopped
    }

    // Generate charts if data exists
    const dataFile = '/tmp/telemetry_data.json';
    if (!fs.existsSync(dataFile)) {
      core.warning('No telemetry data found');
      return;
    }

    core.info('Generating charts...');
    const actionPath = process.env.GITHUB_ACTION_PATH || '.';
    const telemetryBinary = path.join(actionPath, 'telemetry');

    await exec.exec(telemetryBinary, ['--generate-svg', dataFile]);

    // Read data and calculate statistics
    const data = JSON.parse(fs.readFileSync(dataFile, 'utf8'));

    if (data.cpu && data.cpu.length > 0) {
      const cpuLoads = data.cpu.map(s => s.total_load);
      const avgCpu = (cpuLoads.reduce((a, b) => a + b, 0) / cpuLoads.length).toFixed(2);
      const maxCpu = Math.max(...cpuLoads).toFixed(2);

      const memPercents = data.memory.map(s => s.usage_percent);
      const avgMem = (memPercents.reduce((a, b) => a + b, 0) / memPercents.length).toFixed(2);
      const maxMem = Math.max(...memPercents).toFixed(2);

      // Write summary
      await core.summary
        .addHeading('Workflow Telemetry Report')
        .addList([
          `**Data Points**: ${data.cpu.length}`,
          `**CPU Average**: ${avgCpu}%`,
          `**CPU Peak**: ${maxCpu}%`,
          `**Memory Average**: ${avgMem}%`,
          `**Memory Peak**: ${maxMem}%`
        ])
        .write();

      core.info('Charts generated successfully');
    }
  } catch (error) {
    core.warning(`Post action failed: ${error.message}`);
  }
}

post();
