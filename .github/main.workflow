workflow "secsp/ci" {
  on = "push"
  resolves = [
    "secsp/ci/test",
    "secsp/ci/benchmark",
  ]
}

action "secsp/ci/test" {
  uses = "docker://rust:latest"
  runs = "cargo test --all"
}

action "secsp/ci/benchmark" {
  uses = "docker://garyttierney/gh-action-critcmp:latest"
  secrets = ["GITHUB_TOKEN"]
  needs = ["secsp/ci/test"]
}

workflow "secsp/ci-heavy" {
  on = "push"
  resolves = [
    "secsp/ci-heavy/test"
  ]
}

action "secsp/ci-heavy/is-mainline" {
  uses = "actions/bin/filter@master"
  args = "branch master || branch trying || branch staging"
}

action "secsp/ci-heavy/test" {
  uses = "docker://garyttierney/gh-action-afl-fuzz:latest"
  needs = [
    "secsp/ci-heavy/is-mainline",
  ]
  runs = "/bin/sh"
  args = ["-c", "cd /github/workspace/packages/secsp_fuzzer && ./tools/run-fuzzer.sh || exit 1"]
}
