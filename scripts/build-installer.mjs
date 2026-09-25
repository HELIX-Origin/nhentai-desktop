import { execSync } from 'node:child_process';
import { copyFileSync, mkdirSync, chmodSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readFileSync } from 'node:fs';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const pkg = JSON.parse(readFileSync(join(root, 'package.json'), 'utf8'));
const version = pkg.version;
const product = 'NH Desktop';
const binName = 'nh-desktop';

console.log(`Building ${product} release binary...`);
execSync('npx tauri build --no-bundle', { cwd: root, stdio: 'inherit' });

const outDir = join(root, 'dist', 'installer');
mkdirSync(outDir, { recursive: true });

const isWindows = process.platform === 'win32';
let srcBin = join(root, 'src-tauri', 'target', 'release');
if (isWindows) {
  srcBin = join(srcBin, `${binName}.exe`);
} else {
  srcBin = join(srcBin, binName);
}

const platformTag =
  process.platform === 'win32'
    ? `win-${process.arch}`
    : process.platform === 'darwin'
      ? `macos-${process.arch}`
      : `linux-${process.arch}`;
const ext = isWindows ? '.exe' : '';
const versionedName = `${product}-Setup-${version}-${platformTag}${ext}`;
const genericName = `${product}-Setup-${platformTag}${ext}`;

copyFileSync(srcBin, join(outDir, versionedName));
console.log(`Copied -> ${join(outDir, versionedName)}`);

copyFileSync(srcBin, join(outDir, genericName));
if (!isWindows) {
  chmodSync(join(outDir, versionedName), 0o755);
  chmodSync(join(outDir, genericName), 0o755);
}
console.log(`Copied -> ${join(outDir, genericName)}`);
console.log('Done.');