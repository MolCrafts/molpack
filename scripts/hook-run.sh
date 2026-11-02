#!/usr/bin/env bash
# Runs ONE heavy hook command: one that compiles, or runs a heavy test suite.
#
#   scripts/hook-run.sh <command...>
#
# The git hooks no longer call this script. `molci check` hands each
# pre-push check to $MOLCRAFTS_HOOK_RUNNER itself. This file stays because
# the shared githooks look for the string MOLCRAFTS_HOOK_RUNNER here: if it
# is missing they send the whole hook, including the commit stage, to Slurm.
#
# When the environment names a runner in MOLCRAFTS_HOOK_RUNNER and this is not
# already inside a Slurm job, the command is handed to that runner. The
# MolCrafts cluster's shared git hooks set it to a script that runs its
# arguments on a compute node. Everywhere else nothing sets it and the
# command runs right here.
set -euo pipefail
# A git hook exports GIT_DIR, GIT_INDEX_FILE, ... for the hooked repository;
# a heavy gate's own git calls (a partner checkout, uv fetching a git
# dependency) must not inherit them.
unset $(git rev-parse --local-env-vars)
if [ -n "${MOLCRAFTS_HOOK_RUNNER:-}" ] && [ -z "${SLURM_JOB_ID:-}" ]; then
    exec "$MOLCRAFTS_HOOK_RUNNER" "$@"
fi
exec "$@"
