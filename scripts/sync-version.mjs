#!/usr/bin/env node

import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const semverPattern = String.raw`[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?`;

const args = new Set(process.argv.slice(2));
const supportedArgs = new Set(['--check', '--help']);
const unknownArgs = [...args].filter((arg) => !supportedArgs.has(arg));

if (unknownArgs.length > 0) {
  console.error(`Unknown argument${unknownArgs.length > 1 ? 's' : ''}: ${unknownArgs.join(', ')}`);
  process.exit(2);
}

if (args.has('--help')) {
  console.log(`Usage: node scripts/sync-version.mjs [--check]

Synchronize public innertube-rs version references with Cargo.toml.

Options:
  --check  Report stale references without modifying files.
  --help   Show this help message.`);
  process.exit(0);
}

function readCargoVersion() {
  const cargoToml = readFileSync(resolve(repoRoot, 'Cargo.toml'), 'utf8');
  const packageSection = cargoToml.match(/^\[package\]\s*\r?\n([\s\S]*?)(?=^\[)/m);

  if (!packageSection) {
    throw new Error('Could not find the [package] section in Cargo.toml.');
  }

  const versionMatch = packageSection[1].match(/^version\s*=\s*"([^"]+)"\s*(?:#.*)?$/m);
  if (!versionMatch) {
    throw new Error('Could not find package.version in Cargo.toml.');
  }

  const version = versionMatch[1];
  if (!new RegExp(`^${semverPattern}$`).test(version)) {
    throw new Error(`Unsupported Cargo.toml version format: ${version}`);
  }

  return version;
}

const version = readCargoVersion();
const targets = [
  {
    path: 'README.md',
    description: 'Cargo.toml dependency example',
    pattern: new RegExp(`(\\binnertube-rs\\s*=\\s*")${semverPattern}(")`, 'g'),
    replacement: (_match, prefix, suffix) => `${prefix}${version}${suffix}`,
    expectedMatches: 1,
  },
  {
    path: 'site/src/App.tsx',
    description: 'landing-page header badge',
    pattern: new RegExp(`(^\\s*)v${semverPattern}(\\s*$)`, 'gm'),
    replacement: (_match, indentation, trailingWhitespace) =>
      `${indentation}v${version}${trailingWhitespace}`,
    expectedMatches: 1,
  },
];

const pendingUpdates = [];

for (const target of targets) {
  const absolutePath = resolve(repoRoot, target.path);
  const contents = readFileSync(absolutePath, 'utf8');
  const matchCount = [...contents.matchAll(target.pattern)].length;

  if (matchCount !== target.expectedMatches) {
    throw new Error(
      `${target.path}: expected ${target.expectedMatches} ${target.description} match, found ${matchCount}.`,
    );
  }

  const updatedContents = contents.replace(target.pattern, target.replacement);
  if (updatedContents !== contents) {
    pendingUpdates.push({ ...target, absolutePath, updatedContents });
  }
}

if (args.has('--check')) {
  if (pendingUpdates.length > 0) {
    for (const target of pendingUpdates) {
      console.error(`Out of sync: ${target.path} (${target.description})`);
    }
    console.error(`Run "node scripts/sync-version.mjs" to synchronize version ${version}.`);
    process.exit(1);
  }

  console.log(`Version references are in sync with Cargo.toml (${version}).`);
  process.exit(0);
}

for (const target of pendingUpdates) {
  writeFileSync(target.absolutePath, target.updatedContents, 'utf8');
  console.log(`Updated ${target.path}: ${target.description} -> ${version}`);
}

if (pendingUpdates.length === 0) {
  console.log(`Version references are already in sync with Cargo.toml (${version}).`);
}
