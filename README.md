# Marge
Marge is your friendly Merge Assistant. 
She creates MRs/PRs and helps you to assign it to one of your Merge Buddies

init - initialize configs
merge - creates MR

// TODO
- [ ] integrate dialoguer and indicatif
- [ ] create config file and config struct
- [ ] implement the init subcommand


### how should the setup work?
ask for github or gitlab
ask for URL of gitlab or github server
ask for API KEY

ask for project name
ask for default branch

### how shoud the merge command work
after typing marge merge,
marge should look at the diffs, and give you a list of most suited Merge
Reviewers.
after picking the proper Merge Buddy, Marge create a Merge Request with a
HTTP-Request
