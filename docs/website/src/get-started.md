# Get Started

## Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/onagre.svg)](https://repology.org/project/onagre/versions)

### Dependencies

**Backend:**

Onagre uses the [pop-launcher](https://github.com/pop-os/launcher) backend, you will need to install it according to 
your distribution (for arch users there is and AUR package). Alternatively we maintain a fork of pop-launcher called 
[onagre-launcher](https://github.com/onagre-launcher/launcher) which is retro-compatible with the pop-launcher API while 
removing all PopOs specifics. 

**Plugin dependencies:**

If you want to use the default calculator plugin you will need [Qalculate](http://qalculate.github.io/) installed.

### Building from source

If there are no distro package available for Onagre in your preferred manager,
you can build it from source with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

**Latest stable release:** 

```shell
cargo install onagre
```

**Latest upstream:**

```shell
cargo install --git https://github.com/onagre-launcher/onagre
```

## Usage


**1. Key bindings:**


| Key             | Action                       | 
|:----------------|:-----------------------------|
| `Arrow up/down` | Change selection             |
| `Tab`           | Autocomplete (in files mode) | 
| `Esc`           | Quit without launching       | 
| `Enter`         | Launch selection             |


Onagre has three distinct modes: *desktop entries*, *history* and *plugin*. By default, it will start in the *history*
mode which will display previous the most used desktop entry.


**2. Plugins:**

To use a plugin mode simply match its regex when typing your query.

For instance the `file` plugin will match `^(/|~).*`, typing `~/` would enable the plugin and start the file navigation.

Plugin with no prefix are enabled by default, there entry will be mixed in the search results.

**Default plugins:**

| Mode        | Description                                                   | Prefix           | Configuration                                            |
|:------------|:--------------------------------------------------------------|:-----------------|:---------------------------------------------------------|
| History     | Display the most used desktop entries on start                |                  |                                                          |
| PopLauncher | Search for desktop entries                                    |                  |                                                          |
| Pulse       | Control PulseAudio devices and volume                         |                  |                                                          |
| Script      | Shell scripts as launcher options                             |                  | `$HOME/.local/share/pop-launcher/scripts`                |
| Terminal    | Terminal or background commands                               | 'run '           |                                                          | 
| Web         | Web search                                                    | 'ddg ', 'g', ... | `$HOME/.local/share/pop-launcher/plugins/web/config.ron` |
| Files       | Find files using fd/find                                      | 'find '          |                                                          |
| Recent      | Recently-opened document search                               | 'recent '        |                                                          |
| Calc        | Calculator with unit conversion (uses Qalculate! expressions) | '= '             |                                                          |
| Help        | List available pop-launcher modes                             | '?'              |                                                          |


## Configuration and Theming


## CLi

### Launch Onagre in a specific mode

You can prefill the input query using the `--mode` flag. This can be usefull to create a quick shortcut to 
a specific plugin. 

**Example:**

```shell
onagre --mode "run "
```

:::tip
Note that you need to provide the match query corresponding to the plugin configuration,
for most plugin a whitespace is expected after the prefix keyword.
:::

### Scaling

Depending on your desktop environment, screen size, DPI scale it could be usefull to resize onagre without editing your 
whole theme file. To do so use the `--scale` flag.

```shell
onagre --scale 1.2
```

### Launch Onagre with an alternate theme

You can provide a custom location for Onagre theme:

```shell
onagre --theme "/home/me/my_custom_theme.scss"
```

### Theme examples



## Plugins