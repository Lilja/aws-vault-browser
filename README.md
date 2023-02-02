# aws-vault browser
Enables you to run aws-vault profiles in seperate containers/profiles(depending on which browser). Making it possible to run multiple aws-vault sessions simultaneously in different tabs/windows.
For firefox, it runs with multi account containers and the firefox container extensions.
For chrome, it seperates your session with different data/user directories.

## Usage
- `avb login <an-aws-vault-profile>`
- `avb login <an-aws-vault-profile> --container Work --browser firefox`
- `avb list`

## Install
Download from github release. You could use [grm](https://github.com/jsnjack/grm) to install it.

## Requirements
- Firefox
  - [multi account containers](https://addons.mozilla.org/en-US/firefox/addon/multi-account-containers/)
  - [Open external links in a container](https://addons.mozilla.org/en-GB/firefox/addon/open-url-in-container/)
- Chrome
  - None


## Options
It's possible to create a configuration file with what aws-vault profile should be run with which container.
To create a configuration file, please create a toml file in `$XDG_CONFIG_HOME/fav/config.toml` or `~/.config/fav/config.toml`

### Configuration
```toml
profiles = [
    { firefox_container = "stage", aws_vault_profile = "customs-stage"}
]
```

#### Config options
##### profiles
{ firefox_container: string, aws_vault_profile: string }[]

##### firefox_binary_path
A place where a firefox installation can be found.

`firefox_binary_path: string`

*note* the `$BROWSER` environmental variable is also supported.
