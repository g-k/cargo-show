//! `cargo show`

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_qualifications,
    unstable_features
)]

extern crate docopt;
extern crate g_k_crates_io_client as crates_io;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fmt;
use std::process;

static DEFAULT: &'static str = "https://crates.io";

static USAGE: &'static str = r"
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
";

/// Docopt input args.
#[derive(Debug, Deserialize)]
struct Args {
    /// `crate-name`
    arg_crate_name: Vec<String>,
    /// `--version`
    flag_version: bool,
    /// `--json`
    flag_json: bool,
    /// `--dependencies`
    flag_dependencies: bool,
}

/// crate metadata to print
#[derive(Debug, Serialize, Deserialize)]
pub struct CrateMetadata {
    // in response.crate
    created_at: String,
    description: Option<String>,
    documentation: Option<String>,
    downloads: u64,
    homepage: Option<String>,
    id: String,
    keywords: Vec<String>,
    license: Option<String>,
    max_version: String,
    name: String,
    repository: Option<String>,
    updated_at: String,
    versions: Vec<u64>, // also top level keywords and versions arrays of objects
}

/// crate metadata HTTP response
#[derive(Debug, Serialize, Deserialize)]
pub struct CrateMetaResponse {
    #[serde(rename = "crate")]
    crate_data: CrateMetadata,
}

/// crate dependency
#[derive(Debug, Deserialize)]
pub struct CrateDependency {
    // in response.dependencies
    /// The dependent crate's ID.
    #[serde(rename = "crate_id")]
    id: String,

    /// The required version of the dependent crate (semver).
    req: String,

    /// The dependency type.
    ///
    /// Can be: `normal`, `dev` (examples, tests), `build` (deps required by `build.rs`).
    kind: CrateDependencyKind,

    /// The dependency is optional.
    optional: bool,
    // ...
}

/// The dependendy type.
#[derive(Debug, Deserialize, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum CrateDependencyKind {
    /// Just a regular dependency library.
    Normal,
    /// A dependency used in examples or tests.
    Dev,
    /// A dependency used in `build.rs`.
    Build,
}

/// crate dependencies HTTP response
#[derive(Debug, Deserialize)]
pub struct CrateDependencyResponse {
    dependencies: Vec<CrateDependency>,
}

impl fmt::Display for CrateMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "---
id: {id}
name: {name}
description: {description}
documentation: \
                {documentation}
homepage: {homepage}
repository: {repository}
max_version: \
                {max_version}
downloads: {downloads}
license: {license}
created: {created_at}
\
                updated: {updated_at}",
            id = self.id,
            name = self.name,
            description = self.description.as_ref().unwrap_or(&"None".to_owned()),
            documentation = self.documentation.as_ref().unwrap_or(&"None".to_owned()),
            max_version = self.max_version,
            downloads = self.downloads,
            license = self.license.as_ref().unwrap_or(&"None".to_owned()),
            homepage = self.homepage.as_ref().unwrap_or(&"None".to_owned()),
            repository = self.repository.as_ref().unwrap_or(&"None".to_owned()),
            created_at = self.created_at,
            updated_at = self.updated_at
        )
    }
}

impl fmt::Display for CrateDependency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.req)?;
        let isdev = self.kind != CrateDependencyKind::Normal;
        let isopt = self.optional;
        if isdev || isopt {
            write!(f, " (")?;
            if isdev {
                write!(f, "{}", self.kind)?;
            }
            if isdev && isopt {
                write!(f, ", ")?;
            }
            if isopt {
                write!(f, "opt")?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl fmt::Display for CrateDependencyKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            CrateDependencyKind::Normal => "normal",
            CrateDependencyKind::Dev => "dev",
            CrateDependencyKind::Build => "build",
        };
        write!(f, "{}", name)
    }
}

/// fetches and prints package metadata from crates.io
fn print_crate_metadata(crate_name: &str, as_json: bool, with_deps: bool) -> Result<(), String> {
    let mut req = crates_io::Registry::new(DEFAULT.to_string(), None);

    let response = req
        .get_crate_data(crate_name)
        .map_err(|e| format!("Error fetching data for {}: {}", crate_name, e))?;

    let meta: Result<CrateMetaResponse, _> = serde_json::from_str(&response)
        .map_err(|e| format!("Error parsing JSON data for {}: {}", crate_name, e));

    let meta = meta?.crate_data;

    if as_json && with_deps {
        let response = req
            .get_crate_dependencies(&meta.id, &meta.max_version)
            .map_err(|e| format!("Error fetching dependencies for {}: {}", crate_name, e))?;
        println!("{}", response);
        return Ok(());
    }

    if as_json {
        println!("{}", response);
        return Ok(());
    }

    // print crate's metadata
    println!("{}", meta);

    if with_deps {
        println!("dependencies:");

        let response = req
            .get_crate_dependencies(&meta.id, &meta.max_version)
            .map_err(|e| format!("Error fetching dependencies for {}: {}", crate_name, e))?;

        let deps: Result<CrateDependencyResponse, _> = serde_json::from_str(&response)
            .map_err(|e| format!("Error patcing JSON dependencies for {}: {}", crate_name, e));

        let mut deps = deps?.dependencies;
        deps.sort_by(|a, b| a.kind.cmp(&b.kind));
        for dependency in deps {
            println!("{}", dependency);
        }
    }

    Ok(())
}

fn main() {
    let args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize::<Args>())
        .unwrap_or_else(|err| err.exit());

    if args.flag_version {
        println!("cargo-show version {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    for crate_name in &args.arg_crate_name {
        let r = print_crate_metadata(crate_name, args.flag_json, args.flag_dependencies);
        if let Err(e) = r {
            eprintln!("{}", e);
        }
    }
}
