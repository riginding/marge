# Marge
Marge is your friendly Merge Assistant. 
She creates MRs/PRs and helps you to assign it to one of your Merge Buddies

init - initialize configs
merge - creates MR

// TODO
- [x] update to clap
- [ ] implement the init subcommand
- [ ] integrate dialoguer and indicatif
- [ ] create config file and config struct


### how should the init command work?
ask for URL of gitlab server
ask for API KEY
ask for project name
ask for default branch

### how shoud the merge command work?
after typing marge merge,
marge creates a merge request with random merge reviewer

marge should look at the diffs, and give you a list of most suited Merge
Reviewers.
after picking the proper Merge Buddy, Marge create a Merge Request with a
HTTP-Request

### future features
also possible to use with github
