workflow "Run Tests" {
  on = "push"
  resolves = ["Run Cargo Tests"]
}

action "Run Cargo Tests" {
  uses = "./.github/actions/cargo"
  args = "test"
}

workflow "Deploy Binary" {
  on = "push"
  resolves = ["Build Binary"]
}

action "Master Filter" {
  uses = "actions/bin/filter@c6471707d308175c57dfe91963406ef205837dbd"
  args = "branch master"
}

action "Build Binary" {
  uses = "./.github/actions/cargo"
  args = "build"
  needs = ["Master Filter"]
}
