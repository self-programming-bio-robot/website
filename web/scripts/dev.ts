import { spawn } from 'node:child_process';
import { createRequire } from 'node:module';
import path from 'node:path';

import chokidar from 'chokidar';

import { compileTerminalContent, MARKDOWN_ROOT } from './compile-terminal-content';

const require = createRequire(import.meta.url);
const nextPackage = require.resolve('next/package.json');
const nextBinary = path.join(path.dirname(nextPackage), 'dist', 'bin', 'next');
let compiling = false;
let compileQueued = false;

async function compileChangedContent(): Promise<void> {
  if (compiling) {
    compileQueued = true;
    return;
  }

  compiling = true;
  try {
    const changed = await compileTerminalContent();
    console.log(changed ? '[content] regenerated' : '[content] unchanged');
  } catch (error) {
    console.error('[content]', error instanceof Error ? error.message : error);
  } finally {
    compiling = false;
    if (compileQueued) {
      compileQueued = false;
      await compileChangedContent();
    }
  }
}

async function main(): Promise<void> {
  await compileTerminalContent();

  const watcher = chokidar.watch(MARKDOWN_ROOT, {
    ignoreInitial: true,
    ignored: (watchedPath, stats) => Boolean(stats?.isFile() && path.extname(watchedPath) !== '.md'),
  });
  watcher.on('add', compileChangedContent);
  watcher.on('change', compileChangedContent);
  watcher.on('unlink', compileChangedContent);

  const next = spawn(
    process.execPath,
    [nextBinary, 'dev', '--turbopack', ...process.argv.slice(2)],
    { stdio: 'inherit' },
  );

  async function shutdown(signal: NodeJS.Signals): Promise<void> {
    await watcher.close();
    next.kill(signal);
  }

  process.on('SIGINT', () => void shutdown('SIGINT'));
  process.on('SIGTERM', () => void shutdown('SIGTERM'));
  next.on('exit', async (code, signal) => {
    await watcher.close();
    if (signal) {
      process.kill(process.pid, signal);
    } else {
      process.exit(code ?? 0);
    }
  });
}

main().catch((error: unknown) => {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
});
