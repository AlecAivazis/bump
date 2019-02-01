workflow "Deploy version" {
  on = "push"
  resolves = ["Master Filter"]
}

action "Master Filter" {
  uses = "actions/bin/filter@c6471707d308175c57dfe91963406ef205837dbd"
  args = "branch master"
}
