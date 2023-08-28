# Setting up a Repository

A repository contains a bunch of `packagename.toml` files which contains either a `package` or a `collection`.

An example of a repository is [mkanilsson-dotfiles/main](https://github.com/mkanilsson-dotfiles/main)

All files that doesn't end in `.toml` are ignored which allows `README.md` and other files to exist in the repository without problems.

## Packages

A package file includes information about a specific programs config, where it can be found and where it should be installed.

It also contains a list of dependencies.

### How to write a Package

```toml
[Package]
name = "hyprland"
description = "Gruvbox themed hyprland"
repo = { host = "github", owner = "mkanilsson-dotfiles", repo = "hypr" }
install-path = "$HOME/.config/hypr"

dependencies = [
    "scripts",
    "alacritty",
    "rofi",
    "swaync",
    "wallpapers",
    "waybar",
]
```

This package says that it will be cloned from `git@github.com:mkanilsson-dotfiles/hyprland.git` and installed at `~/.config/hypr`

#### TOML

| Name         | Datatype        | Description                                                                              |
|--------------|-----------------|------------------------------------------------------------------------------------------|
| name         | string          | The name of the package                                                                  |
| description  | string          | The description of the package                                                           |
| repo         | repository      | Where the config will be cloned from                                                     |
| install-path | string          | Where the config will be installed. `$HOME` is special as it expands to your home folder |
| dependencies | array of string | Other packages that this package requires to work properly                               |

`Repository` is a special type. `host` can be one of `github`, `gitlab` or `custom`.


In the case of `host = "github"`

| Name  | Datatype | Description                 |
|-------|----------|-----------------------------|
| owner | string   | The owner of the repository |
| repo  | string   | The name of the repository  |

> It will expand to `git@github.com:owner/repo.git`


In the case of `host = "gitlab"`

| Name  | Datatype | Description                 |
|-------|----------|-----------------------------|
| owner | string   | The owner of the repository |
| repo  | string   | The name of the repository  |

> It will expand to `git@gitlab.com:owner/repo.git`


In the case of `host = "custom"`

| Name | Datatype | Description                   |
|------|----------|-------------------------------|
| url  | string   | The url it will be clone from |

> It will expand to `url`

Examples:

```toml
repo = { host = "custom", url = "https://codeberg.org/abinsur/debian-conf" } # This will download it from `https://codeberg.org/abinsur/debian-conf`

repo = { host = "custom", url = "https://github.com/mkanilsson-dotfiles/hyprland" } # This will download it from `https://github.com/mkanilsson-dotfiles` (e.g. not via ssh)
```


## Collections

A collection file includes a list of packages to be installed. 
It could be used if you want a short-hand for installing a bunch of programs related to one subject.

### How to write a Package

```toml
[Collection]
name = "x11"
description = "All-things x11"
packages = [
    "alacritty",
    "dunst",
    "herbstluftwm",
    "picom",
    "polybar",
    "rofi",
    "scripts",
    "wallpapers",
]
```


#### TOML

| Name         | Datatype        | Description                                                                              |
|--------------|-----------------|------------------------------------------------------------------------------------------|
| name         | string          | The name of the package                                                                  |
| description  | string          | The description of the package                                                           |
| packages     | array of string | The packages that this collection will install                                           |

