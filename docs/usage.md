# Usage

## Install

```bash
Install selected packages, will update if already installed

Usage: dotman install [OPTIONS] <PACKAGES>...

Arguments:
  <PACKAGES>...

Options:
  -y, --yes         Skip confirmation
      --force       Force install, this will override existing configurations
      --no-scripts  Don't run .dotman.lua script
  -h, --help        Print help
  -V, --version     Print version
```

Example: 
`dotman install hyprland x11` this will install `hyprland` and its dependencies and all packages specified in the `x11` collection.

> NOTE: `install` and `update` are synonyms

## Update

```bash
Updated selected packages, will install in not already installed

Usage: dotman update [OPTIONS] <PACKAGES>...

Arguments:
  <PACKAGES>...

Options:
  -y, --yes         Skip confirmation
      --force       Force install, this will override existing configurations
      --no-scripts  Don't run .dotman.lua script
  -h, --help        Print help
  -V, --version     Print version
```

Example: 
`dotman update hyprland x11` this will update `hyprland` and its dependencies and all packages specified in the `x11` collection.

> NOTE: `update` and `install` are synonyms

## Install everything

```bash
Install every package avaliable in the repository

Usage: dotman install-everything [OPTIONS]

Options:
  -y, --yes         Skip confirmation
      --force       Force install, this will override existing configurations
      --no-scripts  Don't run .dotman.lua script
  -h, --help        Print help
  -V, --version     Print version
```

Example: 
`dotman install-everything` this will install every package that exist in the repository.

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
`dotman search x1` this will find all packages and collections that, in some way, contains "x1".

> NOTE: All text will be trimmed and comparied in lowercase
