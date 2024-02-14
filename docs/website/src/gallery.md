# Gallery

Don't hesitate to send a PR with your fancy theme, we would be happy to share it to the community.

## Default theme

<img src="/screenshots/default-theme.png" alt="murz-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto;"/>

---

## Hollow

<img src="/screenshots/hollow.png" alt="simple-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto;"/>

::: details expand theme.scss

```scss
.onagre {
  --exit-unfocused: false;
  height: 375px;
  width: 600px;
  --icon-theme: "Papirus";
  --icon-size: 28px;
  --font-family: "Iosevka Nerd Font Mono";
  background: #1c1e26;
  color: #cbced0;
  border-color: #2E3440;
  border-width: 4px;
  border-radius: 8.0%;
  padding: 10px;

  .container {
    padding: 8px;
    .search {
      --spacing: 1;
      background: #cbced0;
      border-radius: 10.0%;
      color: #1c1e26;
      --height: fill-portion 1;

      .plugin-hint {
        font-size: 18px;
        background: #cbced0;
        color: #e95678;
        border-color: #e95678;
        --align-x: center;
        --align-y: center;
        --width: fill-portion 1;
        --height: fill;
      }

      .input {
        font-size: 20px;
        --width: fill-portion 11;
      }
    }

    .rows {
      --height: fill-portion 8;
      border-radius: 8.0%;

      .row-selected {
        background: #268bd2;
        color: #e3e6ee;
        --spacing: 3px;
        --align-y: center;

        border-radius: 8.0%;
        .title {
          font-size: 22px;
        }

        .description {
          font-size: 20px;
        }

        .category-icon {
          --icon-size: 15px;
        }
      }

      .row {
        .title {
          font-size: 22px;
        }

        .description {
          font-size: 20px;
        }

        .category-icon {
          --icon-size: 15px;
        }
      }
    }

    .scrollable {
      background: #00000000;
      .scroller {
        color: #4c566a00;
      }
    }
  }
}
```
:::

---

## Murz

<img src="/screenshots/murz.png" alt="simple-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto;"/>

::: details expand theme.scss

credit to [murz](https://github.com/Murzchnvok/rofi-collection)

```scss
.onagre {
  --exit-unfocused: false;
  height: 250px;
  width: 400px;
  --font-family: "Iosevka,Iosevka Nerd Font";
  font-size: 12px;
  background: #18181b;
  color: #a0a0ab;
  border-color: #5d5e72;
  border-width: 4px;
  padding: 10px;

  .container {
    .search {
      --spacing: 1;
      background: #d8dee9;
      border-radius: 0;
      color: #18181b;
      padding: 4px;
      --height: fill-portion 1;
      .plugin-hint {
        font-size: 9px;
        color: #d8dee9;
        padding: 4px;
        border-width: 2px;
        border-color: #5d5e72;
        background: #18181b;
        --align-x: center;
        --align-y: center;
        --width: fill-portion 2;
        --height: fill;
      }

      .input {
        --width: fill-portion 11;
      }

    }

    .rows {
      --height: fill-portion 6;
      .row-selected {
        background: #20212c;
        color: #5d5e72;
        --spacing: 3px;
      }
    }

    .scrollable {
      .scroller {
        color: #A0A0AB;
        border-color: #18181b;
      }
    }
  }
}
```
:::

---

## Nord

<img src="/screenshots/nord-rounded.png" alt="simple-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto;"/>

::: details expand theme.scss

```scss
.onagre {
  --exit-unfocused: false;
  height: 250px;
  width: 400px;
  --icon-theme: "Papirus";
  --icon-size: 22px;
  --font-family: "Iosevka Nerd Font Mono";
  font-size: 12px;
  background: #2E3440;
  color: #81a1c1;
  border-color: #2E3440;
  border-radius: 25%;
  border-width: 4px;
  padding: 10px;

  .container {
    .search {
      --spacing: 1;
      background: #3b4252;
      color: #d8dee9;
      padding: 4px;
      --height: fill-portion 1;
      .plugin-hint {
        font-size: 9px;
        color: #bf616a;
        padding: 4px;
        border-width: 2px;
        border-color: #bf616a;
        border-radius: 5%;
        background: #4c566a;
        --align-x: center;
        --align-y: center;
        --width: fill-portion 2;
        --height: fill;
      }

      .input {
        --width: fill-portion 11;
      }

    }

    .rows {
      --height: fill-portion 5;
      .row-selected {
        background: #2E3440;
        color: #ebcb8b;
        --spacing: 3px;
        --align-y: center;
      }
    }

    .scrollable {
      .scroller {
        color: #4c566a;
      }
    }
  }
}
```

:::

---

## Not-Adwaita

<img src="/screenshots/not-adwaita.png" alt="simple-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto;"/>

::: details expand theme.scss

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

:::

---

## Solarized

<img src="/screenshots/solarized.png" alt="simple-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto;"/>

::: details expand theme.scss

```scss
.onagre {
  background: #fdf6e3;
  color: #657b83;
  --icon-theme: "Papirus";
  --font-family: "Monaco";
  --icon-size: 24;
  border-radius: 0;
  border-color: #a9b7c6;
  border-width: 0;
  height: 250px;
  width: 440px;

  .container {
    .rows {
      --height: fill-portion 6;
      .row {

        .icon {
          padding-top: 4px;
        }

        .title {
          font-size: 18px;
        }

        .description {
          font-size: 12px;
        }
      }

      .row-selected {
        --width: 435;
        color: #268bd2;

        .icon {
          padding-top: 4px;
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
      background: #fdf6e3;
      --height: fill-portion 1;
      border-radius: 0;
      border-color: #073642;
      border-width: 3px;
      padding: 4px;
      .input {
        color: #002b36;
        --placeholder-color: #657b83;
        --selection-color: #2aa198;
        font-size: 20px;
        --width: fill-portion 13;
      }
      .plugin-hint {
        font-size: 11px;
        color: #002b36;
        padding: 6px;
        border-color: #859900;
        background: #fdf6e3;
        border-width: 3px;
        --align-x: center;
        --align-y: center;
        --width: fill-portion 2;
        --height: fill;
      }
    }

    .scrollable {
      width: 2px;
      background: #839496;
      .scroller {
        border-radius: 0;
        width: 2px;
        color: #268bd2;
      }
    }
  }
}
```

:::
