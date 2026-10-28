// Release checks for the publish job.
//
// In CI this runs via actions/github-script (see .github/workflows/release.yml),
// which provides an authenticated Octokit instance as `github`.

import { execFileSync } from 'node:child_process';

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

// Fails unless the pushed tag points at a commit on `branch` and names the
// version of `crate` in Cargo.toml, so that a stray or mistyped tag stops the
// run before anything is published.
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
}
