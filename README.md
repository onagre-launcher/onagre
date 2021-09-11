# Onagre 

Onagre is a general purpose application launcher for X and wayland  inspired by rofi/wofi and alfred and build with [iced](https://github.com/hecrj/iced/). 

![screenshot](screenshots/sc1.png)

## Disclaimer 

Currently onagre is build on top of a self maintained iced fork, hopefully we will soon be able to 
switch back to the iced master branch (see: [#13](https://github.com/oknozor/onagre/issues/13))

Onagre is at a very early stage of developpement and I have neither the time nor the money to test it on various
distributions/hardware. I am using it on my setup on sway with an AMD GPU. 

That said, I have tested onagre on my current laptop on the following VMs :  
  - **i3** : works fine but you need to resize and add floating via window criteria (see: [i3 user guide](https://i3wm.org/docs/userguide.html)).
  - **gnome** : works but some additional work is needed to position/size the window.
  - **sway** : a fixed size and position are set on startup via `swaymsg`/criteria internaly, this might be removed or exposed in onagre config in a future release.

**Until the roadmap is completed, expect breaking changes, bugs and performance issues.**

## Difference with wofi

I built onagre for my main setup (sway/wayland) as an alternative to [wofi](https://hg.sr.ht/~scoopta/wofi) so it's worth mentioning there are a few differences : 

- Window transparency.
- Rounded corner (this is normally impossible on sway but onagre is wrapped in a transparent iced container 
  allowing to draw the main window with border radius).
- Build with rust and iced.
  Hopefully the choice of a higher level langage (compared to C) and a GUI framework will allow to onboard new contributors easily, without sacrifying to much performance. 
  
Keep in mind that wofi/rofi are much more reliable app launchers and have been around for a while.

## Install

Onagre is currently unreleased, however if you want to give it a shot you can install it with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). Alternatively, if using Arch Linux you can install the git version from the [AUR](https://aur.archlinux.org/packages/onagre-git/).

```
cargo install --git https://github.com/oknozor/onagre
```
## Usage 

### Cli

**1. Alternate config**

You can provide alternate config and theme with the `--config` and `--theme` flags.

**2. Mode selection**

To choose which mode will be available at runtime use the `--modes` flag. 

   ```sh
   onagre --modes xdg drun
   ```

This will start onagre in `xdg` mode (provided it exists in your config) with drun mode available (pressing tab).
If you do not specify any mode onagre will start in drun mode and all config define modes will be available.

For mode info run `onagre --help`.

### Key bindings

| Key     | Action  | 
| :----   | :-----  |
| `Arrow up/down` | Change selection |
| `Tab`   | Switch mode | 
| `Esc`   | Quit without launching | 
| `Enter` | Launch selection | 

## Configure

Onagre configuration is not stabilized yet but you can take a look at the [config example](config_example) directory.

### App config



#### Custom mode

Onagre will look for a config file in `$XDG_CONFIG_DIR/onagre/config.toml`. 
By default it launches in desktop entries mode but you can add additional modes :

```toml
# (Optional) Icon theme, the value must match the theme directory under `$XDG_DATA_DIRS/icons/{my_theme}`
icons = "Arc"

# Example additional mode using `fd` to run xdg open on any file under $HOME
# `source` must be a shell command returning entries separated by `\n`
# `target` is the command to run on the entry, the `%` sign represent the selected entry. 
[modes.xdg]
source = "fd . /home/wololo/"
target = "xdg-open %"

# An other example to integrate `pass` password manager.
# Note that we need to run command in a subshell to escape double quotes and have env variables accessible.
[modes.pass]
source = "sh -c \"cd $HOME/.password-store && fd -t f . | sed s/\\.gpg//\""
target = "sh -c \"pass -c %\""
```
#### Template mode

If you omit the `source` attribute in the mode definition, onagre will display entries history and use your input instead
of the matched entry to launch the command template. 
(located in `$XDG_CACHE_DIR/onagre-{mode_name}`). 

This kind of mode can be used to perform web search :

```toml
[modes.ddg]
target = "xdg-open https://duckduckgo.com/?q=%"
``` 

### Theming

Onagre will look for a theme file in `$XDG_CONFIG_DIR/onagre/theme.toml` and will fall back to the default theme if none is found. 

#### Colors

It simply expose a set of properties available in iced framework. However we handle colors in a slightly different way, using html hex triplet plus two optional hex digits
for opacity. `00` being full transparency, `ff` being fully opaque. If you don't need opacity you can simply omit the last two digits.

```toml
# A fully opaque color
text_color = "#18405a"

# Same color 50% transparent
border_color = "#184057f" 

# Full transparency
border_color = "#00000000" 
```

#### Length and size

You can define a container size using the following properties : 

```toml
width = "fill" # Fill the container
height = "shrink" # Shrink to fit
...

width = "flex-1" # Fill portion (relative to other felx defined size in the container) 
height = "10" # Fixed value
```

To completely hide a menu you can simply set its height and width properties to 0. 

## Roadmap

  - [x] default desktop entries launcher. 
  - [x] optional desktop icons.
  - [x] custom menu from external command.
  - [x] configurable styling.
  - [x] template mode.
  - [x] config from flag.
  - [ ] dmenu mode.
  - [ ] xdg mime support for external commands. 
  - [ ] alfred like workflow.
  - [ ] prefix mode search (ex: type "de" to search for desktop entries).

## Code of conduct

This project is bound by a [code of conduct](CODE_OF_CONDUCT.md) based on the [contributor covenant](https://www.contributor-covenant.org/) if you are not familiar with it, and want to contribute please, read it before going further.

## Contributing

Having a question or suggestion for a new feature ? Feel free to open an issue or submit a PR.
Currently what we need the most is feedback from users using different window managers and hardware. 
If onagre does not work out of the box for you *please let us know* so we can fix it.

## License 

All the code in this repository is released under the MIT License, for more information take a look at the [LICENSE](LICENSE) file.
