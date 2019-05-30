# Marge
Marge is your friendly Merge Assistant. She creates MRs and automatically assigns it to one of your merge buddies

setup - initialize configs
buddy add - add a merge buddy to this project
buddy list - list merge buddies for this git project
merge - creates MR

// TODO
- [ ] create dialoguer autocomplete
- [ ] create sub files
- [ ] create structure for arg passing
- [ ] integrate dialoguer + indicatif
- [ ] create config file & config struct
- [ ] create wizard to create the config struct
- [ ] change git shell commands to git2-rs
- [ ] proper error handling and error messages


### how should the setup work?
ask for github or gitlab
ask for URL of gitlab or github server
ask for API KEY

ask for project name
ask for default branch


### how should the buddy system work
check if you have a config
fetch the users for the project show loading indicator
display a select checkbox list

### how shoud the merge command work
send a http request to create a merge request and assign a random reviewer from the buddy list

