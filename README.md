# Firefox containers & aws-vault


## Install
Download from github. Could use [grm](https://github.com/jsnjack/grm) to install.

## Usage
- `fav login <an-aws-vault-profile>`
- `fav list`

## Configuration
```toml
# Create a toml file in $XDG_CONFIG_HOME/fav/config.toml or ~/.config/fav/config.toml
profiles = [
    { firefox_container = "stage", aws_vault_profile = "customs-stage"}
]
```

### Config options
#### profiles
{ firefox_container: string, aws_vault_profile: string }[]

#### firefox_binary_path
A place where a firefox installation can be found.

`firefox_binary_path: string`

*note* the `$BROWSER` environmental variable is also supported.
