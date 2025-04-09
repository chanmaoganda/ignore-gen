# Command Utility to generate *.gitignore* files
Sources are from [https://github.com/github/gitignore](https://github.com/github/gitignore)

# Usages:
``` shell
Usage: ignore-gen [COMMAND]

Commands:
  generate      Generate gitignore from required language
  list          List all .gitignore templates available on github
  search        Search from github, case insensitive
  regex-search  Search from github, use regex to match languages. !!!Attention!!!, use single quotes in shell, like: '^C.+$' and matches with C prefix
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
## SubCommands:
### generate
ignore-gen generate -t Rust -o .gitignore

### list
ignore-gen list

### search
ignore-gen search C++

### regex-search
ignore-gen regex-search '^C.+$'

!!!Attention!!! Use single quotes if you are using shell
### help
ignore-gen help
