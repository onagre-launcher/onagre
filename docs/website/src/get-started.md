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

If there is no Onagre package available for your linux distribution,
you can build it from source with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

**Latest stable release:** 

```bash
cargo install onagre
```

**Latest upstream:**

```bash
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
mode which will display the most previously used desktop entries.

**2. Plugins:**

To use a plugin mode simply match its regex when typing your query.

For instance the `file` plugin will match `^(/|~).*`, typing `~/` would enable the plugin and start the file navigation.

Plugin with no prefix are enabled by default, there entry will be mixed in the search results.

:::tip
To get help about a plugin usage, just type "?" in Onagre to display the bundled help plugin.
:::

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

Onagre will look for a theme file in `$XDG_CONFIG_DIR/onagre/theme.scss` and will fall back to the default theme if none
is found or if your theme contains syntax errors. To ensure your theme is correctly formatted run `onagre` from the terminal.

A `.scss` extension is used for configuration in order to get syntax highlighting,
but only a small subset of scss is supported along with some custom properties prefixed with `--`.


**Example**:

![not-adwaita.png](/screenshots/not-adwaita.png)

```scss
.onagre {
  background: #d6d6d6;
  color: #000000;
  --icon-theme: "Papirus";
  --font-family: "DejaVuSans";
  --icon-size: 24;
  border-radius: 8%;
  border-color: #d6d6d6;
  border-width: 4px;
  padding: 5px;

  .container {
    .rows {
      --height: fill-portion 6;
      .row {
        --width: 392;

        .icon {
          padding-top: 4px;
        }

        .category-icon {
          padding-left: 5px;
          --icon-size: 11;
        }

        .title {
          font-size: 18px;
        }

        .description {
          font-size: 12px;
        }
      }

      .row-selected {
        --width: 392;
        border-radius: 8%;
        background:  #c0c0c0;

        .icon {
          padding-top: 4px;
        }

        .category-icon {
          padding-left: 5px;
          --icon-size: 11;
        }

        .title {
          font-size: 20px;
        }

        .description {
          font-size: 12px;
        }
      }
    }

    .search {
      border-radius: 5%;
      background: #ffffff;
      --height: fill-portion 1;
      padding: 4px;
      .input {
        font-size: 20px;
      }
    }

    .scrollable {
      width: 2px;
      border-radius: 5%;
      background: #c0c0c0;
      .scroller {
        width: 4px;
        color: #a1a1a1;
      }
    }
  }
}
```

See [Theming -> Reference](theming-reference.md) a detailed explanation of each available property.



## CLi

### Launch Onagre in a specific mode

You can prefill the input query using the `--mode` flag. This can be usefull to create a quick shortcut to 
a specific plugin. 

**Example:**

```bash
onagre --mode "run "
```

:::tip
Note that you need to provide the match query corresponding to the plugin configuration,
for most plugin a whitespace is expected after the prefix keyword.
:::

### Scaling

Depending on your desktop environment, screen size, DPI scale it could be usefull to resize onagre without editing your 
whole theme file. To do so use the `--scale` flag.

```bash
onagre --scale 1.2
```

### Launch Onagre with an alternate theme

You can provide a custom location for Onagre theme:

```bash
onagre --theme "/home/me/my_custom_theme.scss"
```

## Plugins

`pop-launcher` plugins can reside in any of these directories: 

- User-local plugins: `~/.local/share/pop-launcher/plugins/{plugin}/`
- System-wide installation: `/etc/pop-launcher/plugins/{plugin}/`
- Distribution packaging: `/usr/lib/pop-launcher/plugins/{plugin}/`

Every plugin directory is composed of an executable, meant to be called by pop-launcher backend, 
and a config file. 


### Configure existing plugin

Plugin configurations are written with the [Rusty Object Notation](https://github.com/ron-rs/ron) format (aka `.ron`).

For instance, you might customize the default find plugin by editing its config file (`/usr/lib/pop-launcher/plugins/find/plugin.ron)`: 

```ron
(
    // Title of the plugin displayed in the `help` plugin (`?`).
    name: "File search",
    // Description of the plugin displayed in the `help` plugin 
    description: "Syntax: find <filename>\nExample: find my-document.odt",
    query: (
        // Regex which will activate the plugin when matching search input.
        regex: "^(find )+",
        // Syntax example displayed in the `help` plugin.
        help: "find ",
        // Should this plugin mix it's results with the default desktop entry results. 
        isolate: true,
    ),
    // Path of the executable plugin, relative the the plugin directory (you probably don't want to edit this)
    bin: (path: "find"),
    // The icon displayed 
    icon: Name("system-file-manager")
)
```

:::tip
Some plugins may require additional configuration files. 
For instance, the `web` plugin needs a `config.ron` file which allow you to 
configure custom shorthand for web search. 

**Example:**

The following example add the `qw` shorthand to make a web search using Qwant search engine.  
```ron
    (
        matches: [ "qw" ],
        queries: [ (name: "Qwant", query: "https://www.qwant.com/?q=" )]
    ),
    // ..
```

:::

### Install plugins

A variety of plugins are available for Onagre, offering extended functionality and customization options. 
You can explore both community maintained and official plugins on the [awesome-pop-launcher](https://github.com/lucas-dclrcq/awesome-pop-launcher)
repository.

The installation process can vary but most of the time plugin maintainers provide a makefile or a justfile. 
It often boils down to copying the plugin executable and configuration to `~/.local/share/pop-launcher/plugins/{plugin}/`.

### Write your own plugin

Since `pop-launcher` works with JSON IPC over stdin and stdout pipes, you can write your plugin in any language. 

To get started I would suggest looking at a simple example, such as the [emoji plugin](https://github.com/pbui/pop-launcher-scripts)
written in Python by [pbui](https://github.com/pbui).

If you want to build more complex stuff, you might want to take a look at [this blog post](https://oknozor.github.io/blog/write-a-pop-launcher-plugin/)
about writing a Stackoverflow plugin with Rust and the official [onagre-launcher-toolkit](https://docs.rs/onagre-launcher-toolkit/0.1.1/onagre_launcher_toolkit/) crate.

Please if you write your own plugin send a PR to [awesome-pop-launcher](https://github.com/lucas-dclrcq/awesome-pop-launcher) 😊 ! 
