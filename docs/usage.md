# Usage

## Install

```bash
Install selected packages, will update if already installed

Usage: dotman install <PACKAGES>...

Arguments:
  <PACKAGES>...

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Example: 
`dotman install hyprland x11` this will install `hyprland` and its dependencies and all packages specified in the `x11` collection.

> NOTE: `install` and `update` are synonyms

## Update

```bash
Updated selected packages, will install in not already installed

Usage: dotman update <PACKAGES>...

Arguments:
  <PACKAGES>...

Options:
  -h, --help     Print help
  -V, --version  Print version

```

Example: 
`dotman update hyprland x11` this will update `hyprland` and its dependencies and all packages specified in the `x11` collection.

> NOTE: `update` and `install` are synonyms

## Update

```bash
Install every package

Usage: dotman install-everything

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Example: 
`dotman install-everything` this will install every package that exist in the repo.

## Search

```bash
Search for packages and collections

Usage: dotman search <QUERY>

Arguments:
  <QUERY>

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Example:
`dotman search x1` this will find all packages and collections that, in some way, contains "x11".

> NOTE: All text will be trimed and comparied in lowercase
