# Configuration reference

## Classes

### `.onagre`

- **Description:** Root class for styling the Onagre interface.
- **Allowed Attributes:**
    - [`--exit-unfocused`](#exit-unfocused), [`--font-family`](#font-family), [`font-size`](#font-size), 
        [`--icon-theme`](#icon-theme), [`--icon-size`](#icon-size), [`height`](#height), [`width`](#width), 
        [`background`](#background), [`color`](#color), [`border-color`](#border-color), 
        [`border-width`](#border-width), [`border-radius`](#border-radius), [`padding`](#padding), 
        [`padding-left`](#padding-left), [`padding-top`](#padding-top), [`padding-right`](#padding-right), 
        [`padding-bottom`](#padding-bottom)
- **Inner Classes:**
    - [`.container`](#container)

### `.container`

- **Description:** Inner class within `.onagre`, defining styles for the main container.
- **Allowed Attributes:**
    - [`background`](#background), [`color`](#color), [`border-color`](#border-color), [`border-width`](#border-width), 
      [`border-radius`](#border-radius), [`padding`](#padding), [`padding-left`](#padding-left), [`padding-top`](#padding-top), 
      [`padding-right`](#padding-right), [`padding-bottom`](#padding-bottom)
- **Inner Classes:**
    - [`.rows`](#rows)
    - [`.search`](#search)
    - [`.scrollable`](#scrollable)

### `.search`

- **Description:** Class for styling the search bar.
- **Allowed Attributes:**
    - [`background`](#background), [`color`](#color), [`border-color`](#border-color), [`border-radius`](#border-radius), 
      [`border-width`](#border-width), [`padding`](#padding), [`padding-left`](#padding-left), [`padding-right`](#padding-right), 
      [`padding-bottom`](#padding-bottom), [`padding-top`](#padding-top), [`spacing`](#spacing), [`--width`](#--width), 
      [`--height`](#--height), [`--align-x`](#align-x), [`--align-y`](#align-y)
- **Inner Classes:**
    - [`.plugin-hint`](#plugin-hint)
    - [`.bar`](#bar)

### `.plugin-hint`

- **Description:** Class for styling plugin hints.
- **Allowed Attributes:**
    - [`background`](#background), [`color`](#color), [`border-color`](#border-color), 
      [`border-radius`](#border-radius), [`border-width`](#border-width), [`padding`](#padding),
      [`padding-left`](#padding-left), [`padding-right`](#padding-right), [`padding-bottom`](#padding-bottom), 
      [`padding-top`](#padding-top), [`--width`](#--width), [`--height`](#--height), [`--align-x`](#align-x), 
      [`--align-y`](#align-y), [`font-size`](#font-size)

### `.bar`

- **Description:** Class for styling the input bar.
- **Allowed Attributes:**
    - [`background`](#background), [`color`](#color), [`border-color`](#border-color), 
      [`border-radius`](#border-radius), [`border-width`](#border-width), [`text-width`](#text-width), 
      [`selection-color`](#selection-color), [`placeholder-color`](#placeholder-color), [`font-size`](#font-size), 
      [`padding`](#padding), [`padding-left`](#padding-left), [`padding-right`](#padding-right), 
      [`padding-bottom`](#padding-bottom), [`padding-top`](#padding-top), [`--align-x`](#align-x), 
      [`--align-y`](#align-y), [`--width`](#--width), [`--height`](#--height)

### `.rows`

- **Description:** Class for styling rows within a container.
- **Allowed Attributes:**
    - [`background`](#background), [`color`](#color), [`border-color`](#border-color), 
      [`border-radius`](#border-radius), [`border-width`](#border-width), [`padding`](#padding), 
      [`padding-left`](#padding-left), [`padding-top`](#padding-top), [`padding-right`](#padding-right), 
      [`padding-bottom`](#padding-bottom), [`--width`](#--width), [`--height`](#--height)
- **Inner Classes:**
    - [`.row-selected`](#row-selected)
    - [`.row`](#row)

### `.row-selected`

- **Description:** Class for styling the selected row.
- **Allowed Attributes:**
    - [`background`](#background), [`color`](#color), [`border-color`](#border-color), 
      [`border-radius`](#border-radius), [`border-width`](#border-width), [`padding`](#padding), 
      [`padding-left`](#padding-left), [`padding-right`](#padding-right), [`padding-bottom`](#padding-bottom), 
      [`padding-top`](#padding-top), [`--width`](#--width), [`--height`](#--height), [`--align-x`](#align-x), 
      [`--align-y`](#align-y)
- **Inner Classes:**
    - [`.description`](#description)
    - [`.title`](#title)
    - [`.category-icon`](#category-icon)
    - [`.icon`](#icon)

### `.row`

- **Description:** Class for styling default rows.
- **Allowed Attributes:**
    - Similar to `.row-selected` with variations.
- **Inner Classes:**
    - Similar inner classes as `.row-selected`

### `.description`

- **Description:** Class for styling the description section within a row.
- **Allowed Attributes:**
    - Attributes similar to `.row-selected` or `.row`, focused on font size, color, and spacing.

### `.title`

- **Description:** Class for styling the title section within a row.
- **Allowed Attributes:**
    - Attributes similar to `.row-selected` or `.row`, with emphasis on font size and bold styling.

### `.icon` and `.category-icon`

- **Description:** Classes for styling icons within rows.
- **Allowed Attributes:**
    - [`background`](#background), [`color`](#color), [`border-color`](#border-color), 
      [`border-radius`](#border-radius), [`border-width`](#border-width), [`--align-x`](#align-x), 
      [`--align-y`](#align-y), [`--width`](#--width), [`--height`](#--height), [`icon-size`](#icon-size)

### `.scrollable`

- **Description:** Class for styling scrollable areas.
- **Allowed Attributes:**
    - [`background`](#background), [`border-color`](#border-color), [`border-width`](#border-width), [`border-radius`](#border-radius), [`scrollbar-width`](#scrollbar-width), [`scroller`](#scroller), [`scrollbar-margin`](#scrollbar-margin)
  - **Inner Classes:**
    - [`.scroller`](#scroller)

### `.scroller`

- **Description:** Class for styling the scroller within scrollable areas.
- **Allowed Attributes:**
    - [`color`](#color), [`border-color`](#border-color), [`border-width`](#border-width), [`border-radius`](#border-radius), [`scroller-width`](#scroller-width)


## Attributes

### `--exit-unfocused`

- **Descritpion**: weither or not Onagre should exit when it looses focus
- **Value**: Boolean

### `--exit-unfocused`

- **Description:** Whether or not Onagre should exit when it loses focus.
- **Value:** Boolean

### `--font-family`

- **Description:** Defines the font family for styling.
- **Value:** String

### `font-size`

- **Description:** Sets the font size.
- **Value:** Pixel value

### `--icon-theme`

- **Description:** Specifies the theme for icons.
- **Value:** String

### `--icon-size`

- **Description:** Sets the size of icons.
- **Value:** Pixel value

### `height`

- **Description:** Specifies the height on the main window.
- **Value:** Pixel value

### `width`

- **Description:** Specifies the width on the main window.
- **Value:** Pixel value

### `--height`

- **Description:** Specifies the height, with options for pixel value, `fill-portion {int}`, `fill`, or `shrink`.
- **Value:** Pixel value | `fill-portion {int}` | `fill` | `shrink`

### `--width`

- **Description:** Specifies the width, with options for pixel value, `fill-portion`, `fill`, or `shrink`.
- **Value:** Pixel value | `fill-portion` | `fill` | `shrink`

### `background`

- **Description:** Sets the background color.
- **Value:** Color

### `color`

- **Description:** Sets the text color.
- **Value:** Color

### `border-color`

- **Description:** Sets the border color.
- **Value:** Color

### `border-width`

- **Description:** Sets the width of the border.
- **Value:** Pixel value

### `border-radius`

- **Description:** Sets the radius of the border corners.
- **Value:** Percent value

### `padding`

- **Description:** Sets padding.
- **Value:** Pixel value

### `padding-left`

- **Description:** Sets left padding.
- **Value:** Pixel value

### `padding-top`

- **Description:** Sets top padding.
- **Value:** Pixel value

### `padding-right`

- **Description:** Sets right padding.
- **Value:** Pixel value

### `padding-bottom`

- **Description:** Sets bottom padding.
- **Value:** Pixel value

### `spacing`

- **Description:** Sets spacing.
- **Value:** Pixel value

### `--width`

- **Description:** Sets width as a length value.
- **Value:** Length value

### `--height`

- **Description:** Sets height as a length value.
- **Value:** Length value

### `--align-x`

- **Description:** Sets horizontal alignment.
- **Value:**  `left` | `center` | `right`

### `--align-y`

- **Description:** Sets vertical alignment.
- **Value:** `top` | `center` | `bottom`

### `--selection-color`

- **Description:** Sets the color for text selection.
- **Value:** Color

### `--placeholder-color`

- **Description:** Sets the color for placeholder text.
- **Value:** Color

### `--text-width`

- **Description:** Sets the width of the text area as a length value.
- **Value:** Length value

### `--scrollbar-width`

- **Description:** Sets the width of the scrollbar in pixels.
- **Value:** Pixel value

### `--scroller`

- **Description:** Specific block for scroller styles.
- **Value:** Varies

### `--scrollbar-margin`

- **Description:** Sets the margin around the scrollbar in pixels.
- **Value:** Pixel value

### `--scroller-width`

- **Description:** Sets the width of the scroller in pixels.
- **Value:** Pixel value