#!/usr/bin/env bash

## Helper script to run cargo show and update the README.md
# run from project root

# "cargo run show" for development or "cargo show" for a published version
CARGO_CMD="cargo run show"
cargo uninstall cargo-show || true
cargo install cargo-show &> ./cargo-show-install.log

cat > README.md <<EOF
## cargo-show

[![crates.io version](https://img.shields.io/crates/v/cargo-show.svg)](https://img.shields.io/crates/v/cargo-show.svg)
[![Build Status](https://travis-ci.org/g-k/cargo-show.svg?branch=master)](https://travis-ci.org/g-k/cargo-show)
[![Build status](https://ci.appveyor.com/api/projects/status/m9cf5vhft7qwisas?svg=true)](https://ci.appveyor.com/project/g-k/cargo-show)

Prints package metadata like pip show, apt-cache show, npm view, gem query, etc.

To install:

\`\`\`sh
\$ cargo install cargo-show
$(head -n5 ./cargo-show-install.log)
...
$(tail -n5 ./cargo-show-install.log)
\$
\`\`\`

Usage:

\`\`\`sh
\$ cargo show --help
$($CARGO_CMD --help)
\`\`\`

To print package metadata:

\`\`\`sh
\$ cargo show nonexistent-package servo
$($CARGO_CMD webrender servo 2>&1)
\`\`\`

To print JSON:

\`\`\`json
\$ cargo show --json serde | cut -b '1-120'
$($CARGO_CMD --json serde | cut -b '1-120')
\`\`\`

To print package metadata and direct dependencies (alternatively use \`-L\`):

\`\`\`sh
\$ cargo show --dependencies time
$($CARGO_CMD --dependencies time 2>&1)
\`\`\`


To print package metadata and direct dependencies as JSON:

\`\`\`sh
\$ cargo show --dependencies --json time | python -m json.tool | head -n25
$($CARGO_CMD --dependencies --json time | python -m json.tool | head -n25)
\`\`\`


To rename the command if you're used to other package managers:

\`\`\`sh
\$ cd /usr/local/bin/  # or someplace in path
\$ ln \$(which cargo-show) cargo-flizblorp  # needs to be a hardlink
\$ cargo --list | grep fliz
    flizblorp
\`\`\`

### Maintainers

* [@g-k](https://github.com/g-k)
* [@pravic](https://github.com/pravic)

### Contributors

* [@g-k](https://github.com/g-k)
* [@leoschwarz](https://github.com/leoschwarz)
* [@pravic](https://github.com/pravic)
EOF

rm ./cargo-show-install.log

## if the readme changed cargo-show is broken or metadata from the
## example crates changed
git diff
