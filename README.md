## cargo-show

[![crates.io version](https://img.shields.io/crates/v/cargo-show.svg)](https://img.shields.io/crates/v/cargo-show.svg)
[![Build Status](https://travis-ci.org/g-k/cargo-show.svg?branch=master)](https://travis-ci.org/g-k/cargo-show)
[![Build status](https://ci.appveyor.com/api/projects/status/m9cf5vhft7qwisas?svg=true)](https://ci.appveyor.com/project/g-k/cargo-show)

Prints package metadata like pip show, apt-cache show, npm view, gem query, etc.

To install:

```sh
$ cargo install cargo-show
    Updating crates.io index
  Installing cargo-show v0.5.7
 Downloading crates ...
  Downloaded lazy_static v1.4.0
  Downloaded strsim v0.9.3
...
   Compiling g-k-crates-io-client v0.27.1
   Compiling cargo-show v0.5.7
    Finished release [optimized] target(s) in 2m 30s
  Installing /Users/greg/.cargo/bin/cargo-show
   Installed package `cargo-show v0.5.7` (executable `cargo-show`)
$
```

Usage:

```sh
$ cargo show --help
Usage:
    cargo show [options] <crate-name>...
    cargo show (-h|--help)
    cargo show --version

Options:
    --json                  Print the JSON response.
    -L --dependencies       Print the crate's dependencies as well.
    -h --help               Show this help page.
    --version               Show version.

Display a metadata for a create at crates.io.
```

To print package metadata:

```sh
$ cargo show nonexistent-package servo
Error fetching data for nonexistent-package: api errors (status 404 Not Found): Not Found
---
id: servo
name: servo
description: Parked non-servo thing
documentation: None
homepage: None
repository: None
max_version: 0.0.1
downloads: 3137
license: None
created: 2014-12-04T23:41:05.915728+00:00
updated: 2015-12-11T23:55:55.315022+00:00
```

To print JSON:

```json
$ cargo show --json serde | cut -b '1-120'
{"crate":{"id":"serde","name":"serde","updated_at":"2019-12-16T04:09:49.363249+00:00","versions":[196666,192011,185512,1
```

To print package metadata and direct dependencies (alternatively use `-L`):

```sh
$ cargo show --dependencies time
---
id: time
name: time
description: Date and time library. Fully interoperable with the standard library. Mostly compatible with #![no_std].
documentation: None
homepage: None
repository: https://github.com/time-rs/time
max_version: 0.2.2
downloads: 10771986
license: None
created: 2014-11-13T06:52:51.369245+00:00
updated: 2020-01-08T03:27:19.481880+00:00
dependencies:
serde ^1 (opt)
```


To print package metadata and direct dependencies as JSON:

```sh
$ cargo show --dependencies --json time | python -m json.tool | head -n25
{
    "dependencies": [
        {
            "crate_id": "serde",
            "default_features": false,
            "downloads": 0,
            "features": [
                "derive",
                "alloc"
            ],
            "id": 1078018,
            "kind": "normal",
            "optional": true,
            "req": "^1",
            "target": null,
            "version_id": 202181
        }
    ]
}
```


To rename the command if you're used to other package managers:

```sh
$ cd /usr/local/bin/  # or someplace in path
$ ln $(which cargo-show) cargo-flizblorp  # needs to be a hardlink
$ cargo --list | grep fliz
    flizblorp
```

### Maintainers

* [@g-k](https://github.com/g-k)
* [@pravic](https://github.com/pravic)

### Contributors

* [@g-k](https://github.com/g-k)
* [@leoschwarz](https://github.com/leoschwarz)
* [@pravic](https://github.com/pravic)
