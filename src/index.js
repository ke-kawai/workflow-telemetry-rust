const core = require('@actions/core');
const exec = require('@actions/exec');
const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

async function run() {
  try {
    const interval = core.getInput('interval') || '2';

    core.info('Starting telemetry monitoring...');

    // Get action path
    const actionPath = process.env.GITHUB_ACTION_PATH || '.';
    const telemetryBinary = path.join(actionPath, 'telemetry');

    // Make binary executable
    await exec.exec('chmod', ['+x', telemetryBinary]);

    // Start monitoring in background using spawn
    const child = spawn(telemetryBinary, [], {
      detached: true,
      stdio: 'ignore',
      env: {
        ...process.env,
        TELEMETRY_INTERVAL: interval,
        TELEMETRY_ITERATIONS: '999999'
      }
    });

    // Unref so parent can exit
    child.unref();

    // Get PID and save it
    const pid = child.pid;
    const pidFile = '/tmp/telemetry.pid';
    fs.writeFileSync(pidFile, pid.toString());

    // Save PID to state for post action
    core.saveState('telemetry-pid', pid);

    core.info(`Telemetry monitoring started (PID: ${pid}, interval: ${interval}s)`);
  } catch (error) {
    core.setFailed(`Action failed: ${error.message}`);
  }
}

run();
