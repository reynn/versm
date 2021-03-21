# Versm <!-- omit in toc -->

There are a lot of "version" managers out there such as [NVM][nvm], [Gimme][gimme] and [Pyenv][pyenv] to name a few. The goal here is to create a system like that for various languages as well as devtools we commonly use such as [exa][exa], [Ripgrep][ripgrep] and [hyperfine][hyperfine] that release packages releases via [GitHub][github] and others.

- [1. Architecture](#1-architecture)
- [2. Features](#2-features)
- [3. Directory structure](#3-directory-structure)
- [4. Usage examples](#4-usage-examples)
- [5. Code Structure](#5-code-structure)
  - [5.1. Core](#51-core)
  - [5.2. Github](#52-github)
  - [5.3. Golang](#53-golang)
  - [5.4. NodeJS](#54-nodejs)
- [6. Building](#6-building)
- [7. Packaging](#7-packaging)
- [8. Releasing](#8-releasing)
- [9. References](#9-references)

## 1. Architecture

View the [Architecture.md][architecture] for an overview on the code and architecture of the application.

## 2. Features

- Manage binaries from GitHub or language installs for Go/NodeJS/etc
- Config file to define what repos and languages to manage as well as upgrade paths
- (Planned) Directory aware: When `cd` to a directory change to a subenv for that directory that has tools for that project

## 3. Directory structure

By default will check for the [XDG][xdg] directory structure for storing application data.

```
${XDG_DATA_DIR:-$HOME}/.versm/
+-- .bins/
|   +-- exa -> ../github/ogham/exa/v0.9.0/exa-macos-x86_64
|   +-- fd -> ../github/ogham/exa/v0.9.0/exa-macos-x86_64
+-- github/
|   +-- ogham/exa/v0.9.0/
|       +-- exa-macos-x86_64-0.9.0.zip
|       +-- exa-macos-x86_64
|   +-- junegunn/fzf/
|       + 0.21.0/
|       + 0.22.0/
|       + latest -> 0.22.0
|   +-- sharkdp/fd/v8.1.1/
|       +-- fd-v8.1.1-x86_64-apple-darwin.tar.gz # Archives get extracted to the current version directory
|       +-- ....
|       +-- autocomplete/
|       +-- fd
+-- golang/
+-- nodejs/
|   +-- 12.18.3/
|   +-- 14.8.0/
+-- python/
|   +-- 3.7.8/
|   +-- 3.8.5/
```

## 4. Usage examples

Manage the releases for `sharkdp/fd` repo from GitHub

```console
# Install the latest version of sharkdp's fd tool from GitHub
$ versm install --type gh --version latest sharkdp/fd
Found 23 versions listed on GitHub
Latest version is :: v8.1.1
Downloading...
Extracting...
Linking...
Ready to use!

# List installed versions for a specific repository
$ versm list sharkdp/fd
v8.1.1 <- Current (Latest)
v8.1.0
v8.0.0
v8.1.1

# Update all releases
$ versm update
Checking for updates to 4 releases
GitHub
------
sharkdp/fd
  - v8.1.1 <- Current
  - v8.1.0
  - v8.0.0
  - v8.1.1
BurntSushi/ripgrep
  - 12.0.0 <- Current
alacritty/alacritty
  - v0.5.0 <- Current
nushell/nushell
  - 0.17.0 <- Current
...
Found updates for the following
-------------------------------
nushell/nushell: 0.17.0 -> 0.18.1
BurntSushi/ripgrep: 12.0.0 -> 12.1.1

# Remove a specific version of
$ versm remove gh:sharkdp/fd v8.1.1
Successfully removed GitHub release sharkdp/fd v8.1.1

# Show details of a release
# Description will show as rendered markedown instead of code
$ versm show gh:sharkdp/fd@latest
Details on latest Github Release for sharkdp/fd@v8.1.1
Tag: v8.1.1
SHA: 5648597a617adae62500c65320f377574a1325fd
Global ID: MDY6Q29tbWl0OTA3OTM0MTg6NTY0ODU5N2E2MTdhZGFlNjI1MDBjNjUzMjBmMzc3NTc0YTEzMjVmZA
Released On: 2020-05-25
Description:
  ## Bugfixes

  - Support colored output on older Windows versions if either (1) `--color=always` is set or (2) the `TERM` environment variable is set. See #469
Assets: {
  [
    name: fd-v8.1.1-i686-pc-windows-gnu.zip
    from: https://github.com/sharkdp/fd/releases/download/v8.1.1/fd-v8.1.1-i686-pc-windows-gnu.zip
    installed: false
  ],
  [
    name: fd-v8.1.1-i686-pc-windows-msvc.zip
    from: https://github.com/sharkdp/fd/releases/download/v8.1.1/fd-v8.1.1-i686-pc-windows-msvc.zip
    installed: false
  ],
  [
    name: fd-musl_8.1.1_armhf.deb
    from: https://github.com/sharkdp/fd/releases/download/v8.1.1/fd-musl_8.1.1_armhf.deb
    installed: false
  ],
  [
    name: fd-v8.1.1-arm-unknown-linux-gnueabihf.tar.gz
    from: https://github.com/sharkdp/fd/releases/download/v8.1.1/fd-v8.1.1-arm-unknown-linux-gnueabihf.tar.gz
    installed: false
  ],
  [
    name: fd_8.1.1_armhf.deb
    from: https://github.com/sharkdp/fd/releases/download/v8.1.1/fd_8.1.1_armhf.deb
    installed: false
  ],
  [
    name: fd-v8.1.1-x86_64-apple-darwin.tar.gz
    from: https://github.com/sharkdp/fd/releases/download/v8.1.1/fd-v8.1.1-x86_64-apple-darwin.tar.gz
    installed: true
  ]
}

```

## 5. Code Structure

### 5.1. Core

- Tool: A dev tool being managed by Versm
  - Examples:
    - GitHubRelease:
    - Golang:
    - NodeJS:
- Version:
  - Data:
    - tool: A reference to the tool
    - git_info: git tag information
      - sha: Full sha of git commit the tag points to
      - tag_name:
- VersionHandler: A trait that translate the various tools management tools
  - Traits
- Extractors
  - Manage extracting archives when necessary
  - If a download manager sends a release entry to the extractors it will check file metadata
  - if an archive of some type is in the assets it will see if an available extractor is available
  - extractors in core will eventually include zip,tar.*,7zip
  - extractors will likely start of by spawning processes unless there is are pre-existing R]]]ust bindings
- Linkers
  - a Linker is something that handles symlinking or equivalent within the directory structure
- Downloader
  - Managers for downloading files
  - Likely implementations for writing to File, possibly some implementations for things like Cloud Storage providers like S3/Google Drive/etc.
  - Queues
    - Should include niceties such as waiting between queue items,
    - potential implementations?:
      - parallel: Takes the results from the queue and spawns a task to handle downloading, splits throughput between multiple downloads
      - bounded parallel: Same as parallel but only spawns up to a certain amount of tasks before waiting for a spot to open up
      - serial: Executes downloads one at a time. Max throughput on a single item at a time

### 5.2. Github

### 5.3. Golang

### 5.4. NodeJS

## 6. Building

## 7. Packaging

## 8. Releasing

Releases to the main repo and [crates.io][crates-io] will be handled with a [GitHub Actions Workflow]()

## 9. References

This application is authored using [abscissa], a Rust application framework.

[nvm]: https://github.com/nvm-sh/nvm
[gimme]: https://github.com/travis-ci/gimme
[pyenv]: https://github.com/pyenv/pyenv
[exa]: https://github.com/ogham/exa
[ripgrep]: https://github.com/BurntSushi/ripgrep
[hyperfine]: https://github.com/sharkdp/hyperfine
[github]: https://github.com
[xdg]: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
[architecture]: ARCHITECTURE.md
[abscissa]: https://github.com/iqlusioninc/abscissa
[crates-io]: https://crates.io