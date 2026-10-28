import assert from 'node:assert/strict';
import { test } from 'node:test';
import { releaseNotes, tagName } from './release.ts';

const changelog = `# Changelog

## [Unreleased]

## [v0.2.0] - 2026-02-01

### Added

* Two.

## [v0.1.0] - 2026-01-01

* One.

[Unreleased]: https://example.com/compare/v0.2.0...master
[v0.2.0]: https://example.com/compare/v0.1.0...v0.2.0
[v0.1.0]: https://example.com/compare/v0.0.0...v0.1.0
`;

test('releaseNotes returns the section and compare link', () => {
  assert.equal(
    releaseNotes(changelog, 'v0.2.0'),
    '### Added\n\n* Two.\n\n**Diff**: https://example.com/compare/v0.1.0...v0.2.0\n',
  );
});

test('releaseNotes stops the last section at the link definitions', () => {
  assert.equal(
    releaseNotes(changelog, 'v0.1.0'),
    '* One.\n\n**Diff**: https://example.com/compare/v0.0.0...v0.1.0\n',
  );
});

test('releaseNotes rejects a missing section', () => {
  assert.throws(() => releaseNotes(changelog, 'v0.3.0'), /has no v0.3.0 section/);
});

test('releaseNotes rejects an empty section', () => {
  assert.throws(() => releaseNotes(changelog, 'Unreleased'), /has an empty Unreleased section/);
});

test('releaseNotes rejects a missing compare link', () => {
  const unlinked = changelog.replace(/^\[v0\.2\.0\]: .*\n/m, '');
  assert.throws(() => releaseNotes(unlinked, 'v0.2.0'), /has no \[v0.2.0\] compare link/);
});

test('tagName strips refs/tags/', () => {
  assert.equal(tagName('refs/tags/v0.15.0'), 'v0.15.0');
});

test('tagName rejects branch refs', () => {
  assert.throws(() => tagName('refs/heads/master'), /is not a tag/);
});
