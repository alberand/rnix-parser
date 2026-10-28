import assert from 'node:assert/strict';
import { test } from 'node:test';
import { tagName } from './release.ts';

test('tagName strips refs/tags/', () => {
  assert.equal(tagName('refs/tags/v0.15.0'), 'v0.15.0');
});

test('tagName rejects branch refs', () => {
  assert.throws(() => tagName('refs/heads/master'), /is not a tag/);
});
