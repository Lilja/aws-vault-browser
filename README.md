# aws-vault browser
Enables you to run aws-vault profiles in seperate containers/profiles(depending on which browser). Making it possible to run multiple aws-vault sessions simultaneously in different tabs/windows.
For firefox, it runs with multi account containers and the firefox container extensions.

## Usage
- `avb --browser_path <path-to-cli-binary> login --aw_profile <an-aws-vault-profile> --b_container <firefox-multi-account-container>`
- `avb login --aw_profile Work --b_container firefox`

## Install
Download from github release. You could use [grm](https://github.com/jsnjack/grm) to install it.

## Requirements
- Firefox
  - [multi account containers](https://addons.mozilla.org/en-US/firefox/addon/multi-account-containers/)
  - [Open external links in a container](https://addons.mozilla.org/en-GB/firefox/addon/open-url-in-container/)



## Setup
In your shell config:
```
alias work-stage="avb --browser_path /Applications/Firefox.app/Contents/MacOS/firefox login -b firefox -c Stage -p work-stage"
```

## Dev
n/a
