# cosmic-theme-import

A simple, lightweight CLI utility to import and export COSMIC desktop themes.

This tool allows you to import a `.ron` COSMIC theme file (e.g. [COSMIC Themes](https://cosmic-themes.org/)) without being on COSMIC.

## Install

Script:

```sh
curl -sSL https://raw.githubusercontent.com/ByteAtATime/cosmic-theme-import/main/install.sh | sh
```

Or with [cargo](https://rustup.rs):

```sh
cargo install --git https://github.com/ByteAtATime/cosmic-theme-import
```

Or grab a binary directly from [releases](https://github.com/ByteAtATime/cosmic-theme-import/releases) and put it somewhere on your `PATH` (e.g. `~/.local/bin`).

## Usage

```sh
# Import a theme and switch the system to light/dark mode to match it
cosmic-theme-import import ~/Downloads/MyTheme.ron

# Import without touching the system light/dark setting
cosmic-theme-import import ~/Downloads/MyTheme.ron --no-switch-mode

# Export the currently active theme to ActiveTheme.ron
cosmic-theme-import export ActiveTheme.ron

# Export a specific mode regardless of what's active
cosmic-theme-import export ActiveTheme.ron --dark
cosmic-theme-import export ActiveTheme.ron --light
```

Run `cosmic-theme-import --help` for all options.
