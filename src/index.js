const core = require('@actions/core');
const exec = require('@actions/exec');
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

    // Start monitoring in background
    const child = exec.exec(
      telemetryBinary,
      [],
      {
        env: {
          ...process.env,
          TELEMETRY_INTERVAL: interval,
          TELEMETRY_ITERATIONS: '999999'
        },
        ignoreReturnCode: true,
        detached: true,
        stdio: ['ignore', 'ignore', 'ignore']
      }
    );

    // Get PID and save it
    const pid = child.pid || process.pid;
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
