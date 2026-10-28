// Release checks and GitHub release creation for the publish job.
//
// In CI this runs via actions/github-script (see .github/workflows/release.yml),
// which provides an authenticated Octokit instance as `github`.

import { execFileSync } from 'node:child_process';
import fs from 'node:fs';

type GitHub = InstanceType<typeof import('@actions/github/lib/utils').GitHub>;
type Context = typeof import('@actions/github').context;

// Returns the tag name of a `refs/tags/...` ref.
export function tagName(ref: string): string {
  const prefix = 'refs/tags/';
  if (!ref.startsWith(prefix)) {
    throw new Error(`${ref} is not a tag`);
  }
  return ref.slice(prefix.length);
}

// Returns the release notes for `tag` from a Keep a Changelog file: the body
// of its `## [tag]` section followed by its compare link.
export function releaseNotes(changelog: string, tag: string): string {
  const lines = changelog.split('\n');
  const start = lines.findIndex((line) => line.startsWith(`## [${tag}]`));
  if (start === -1) {
    throw new Error(`CHANGELOG.md has no ${tag} section`);
  }
  // The section ends at the next version heading or at the link definitions.
  const end = lines.findIndex((line, i) => i > start && /^(## \[|\[[^\]]+\]: )/.test(line));
  const body = lines.slice(start + 1, end === -1 ? undefined : end).join('\n').trim();
  if (body === '') {
    throw new Error(`CHANGELOG.md has an empty ${tag} section`);
  }

  const link = lines.find((line) => line.startsWith(`[${tag}]: `));
  if (!link) {
    throw new Error(`CHANGELOG.md has no [${tag}] compare link`);
  }

  return `${body}\n\n**Diff**: ${link.slice(`[${tag}]: `.length)}\n`;
}

// Returns the version of `crate` in the Cargo workspace in the current directory.
function cargoVersion(crate: string): string {
  const metadata = JSON.parse(
    execFileSync('cargo', ['metadata', '--no-deps', '--format-version', '1'], { encoding: 'utf8' }),
  );
  const pkg = metadata.packages.find((p: { name: string }) => p.name === crate);
  if (!pkg) {
    throw new Error(`Cargo.toml has no ${crate} package`);
  }
  return pkg.version;
}

// Fails unless the pushed tag points at a commit on `branch`, names the
// version of `crate` in Cargo.toml, and has release notes in CHANGELOG.md, so
// that a stray or mistyped tag stops the run before anything is published.
export async function checkRelease({ github, context, branch, crate }: {
  github: GitHub;
  context: Context;
  branch: string;
  crate: string;
}): Promise<void> {
  const { owner, repo } = context.repo;
  const tag = tagName(context.ref);

  const { status } = (
    await github.rest.repos.compareCommits({ owner, repo, base: context.sha, head: branch })
  ).data;
  if (status !== 'ahead' && status !== 'identical') {
    throw new Error(`${tag} is not on ${branch}`);
  }

  const version = cargoVersion(crate);
  if (tag !== `v${version}`) {
    throw new Error(`${tag} does not match version ${version} of ${crate} in Cargo.toml`);
  }

  releaseNotes(fs.readFileSync('CHANGELOG.md', 'utf8'), tag);
}

// Creates the GitHub release for the pushed tag, with its CHANGELOG.md
// section as the notes.
export async function createRelease({ github, context }: {
  github: GitHub;
  context: Context;
}): Promise<void> {
  const { owner, repo } = context.repo;
  const tag = tagName(context.ref);
  await github.rest.repos.createRelease({
    owner,
    repo,
    tag_name: tag,
    name: tag,
    body: releaseNotes(fs.readFileSync('CHANGELOG.md', 'utf8'), tag),
  });
}
