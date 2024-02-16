import{_ as p}from"./not-adwaita-D2Y_w_wh.js";import{_ as t,r as e,o as c,c as r,a as s,b as n,d as h,e as a}from"./app-Dj7meDOY.js";const o="/screenshots/default-theme.png",i="/screenshots/hollow.png",u="/screenshots/murz.png",j="/screenshots/nord-rounded.png",b="/screenshots/solarized.png",d={},m=a('<h1 id="gallery" tabindex="-1"><a class="header-anchor" href="#gallery"><span>Gallery</span></a></h1><p>Don&#39;t hesitate to send a PR with your fancy theme, we would be happy to share it to the community.</p><h2 id="default-theme" tabindex="-1"><a class="header-anchor" href="#default-theme"><span>Default theme</span></a></h2><img src="'+o+'" alt="murz-theme-screenshot" style="display:block;margin-left:auto;margin-right:auto;"><hr><h2 id="hollow" tabindex="-1"><a class="header-anchor" href="#hollow"><span>Hollow</span></a></h2><img src="'+i+`" alt="simple-theme-screenshot" style="display:block;margin-left:auto;margin-right:auto;"><details class="custom-container details"><summary>expand theme.scss</summary><div class="language-scss" data-ext="scss" data-title="scss"><pre class="language-scss"><code><span class="hljs-selector-class">.onagre</span> {
  <span class="hljs-attr">--exit-unfocused</span>: false;
  <span class="hljs-attribute">height</span>: <span class="hljs-number">375px</span>;
  <span class="hljs-attribute">width</span>: <span class="hljs-number">600px</span>;
  <span class="hljs-attr">--icon-theme</span>: <span class="hljs-string">&quot;Papirus&quot;</span>;
  <span class="hljs-attr">--icon-size</span>: <span class="hljs-number">28px</span>;
  <span class="hljs-attr">--font-family</span>: <span class="hljs-string">&quot;Iosevka Nerd Font Mono&quot;</span>;
  <span class="hljs-attribute">background</span>: <span class="hljs-number">#1c1e26</span>;
  <span class="hljs-attribute">color</span>: <span class="hljs-number">#cbced0</span>;
  <span class="hljs-attribute">border-color</span>: <span class="hljs-number">#2E3440</span>;
  <span class="hljs-attribute">border-width</span>: <span class="hljs-number">4px</span>;
  <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">8.0%</span>;
  <span class="hljs-attribute">padding</span>: <span class="hljs-number">10px</span>;

  <span class="hljs-selector-class">.container</span> {
    <span class="hljs-attribute">padding</span>: <span class="hljs-number">8px</span>;
    <span class="hljs-selector-class">.search</span> {
      <span class="hljs-attr">--spacing</span>: <span class="hljs-number">1</span>;
      <span class="hljs-attribute">background</span>: <span class="hljs-number">#cbced0</span>;
      <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">10.0%</span>;
      <span class="hljs-attribute">color</span>: <span class="hljs-number">#1c1e26</span>;
      <span class="hljs-attr">--height</span>: fill-portion <span class="hljs-number">1</span>;

      <span class="hljs-selector-class">.plugin-hint</span> {
        <span class="hljs-attribute">font-size</span>: <span class="hljs-number">18px</span>;
        <span class="hljs-attribute">background</span>: <span class="hljs-number">#cbced0</span>;
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#e95678</span>;
        <span class="hljs-attribute">border-color</span>: <span class="hljs-number">#e95678</span>;
        <span class="hljs-attr">--align-x</span>: center;
        <span class="hljs-attr">--align-y</span>: center;
        <span class="hljs-attr">--width</span>: fill-portion <span class="hljs-number">1</span>;
        <span class="hljs-attr">--height</span>: fill;
      }

      <span class="hljs-selector-class">.input</span> {
        <span class="hljs-attribute">font-size</span>: <span class="hljs-number">20px</span>;
        <span class="hljs-attr">--width</span>: fill-portion <span class="hljs-number">11</span>;
      }
    }

    <span class="hljs-selector-class">.rows</span> {
      <span class="hljs-attr">--height</span>: fill-portion <span class="hljs-number">8</span>;
      <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">8.0%</span>;

      <span class="hljs-selector-class">.row-selected</span> {
        <span class="hljs-attribute">background</span>: <span class="hljs-number">#268bd2</span>;
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#e3e6ee</span>;
        <span class="hljs-attr">--spacing</span>: <span class="hljs-number">3px</span>;
        <span class="hljs-attr">--align-y</span>: center;

        <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">8.0%</span>;
        <span class="hljs-selector-class">.title</span> {
          <span class="hljs-attribute">font-size</span>: <span class="hljs-number">22px</span>;
        }

        <span class="hljs-selector-class">.description</span> {
          <span class="hljs-attribute">font-size</span>: <span class="hljs-number">20px</span>;
        }

        <span class="hljs-selector-class">.category-icon</span> {
          <span class="hljs-attr">--icon-size</span>: <span class="hljs-number">15px</span>;
        }
      }

      <span class="hljs-selector-class">.row</span> {
        <span class="hljs-selector-class">.title</span> {
          <span class="hljs-attribute">font-size</span>: <span class="hljs-number">22px</span>;
        }

        <span class="hljs-selector-class">.description</span> {
          <span class="hljs-attribute">font-size</span>: <span class="hljs-number">20px</span>;
        }

        <span class="hljs-selector-class">.category-icon</span> {
          <span class="hljs-attr">--icon-size</span>: <span class="hljs-number">15px</span>;
        }
      }
    }

    <span class="hljs-selector-class">.scrollable</span> {
      <span class="hljs-attribute">background</span>: <span class="hljs-number">#00000000</span>;
      <span class="hljs-selector-class">.scroller</span> {
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#4c566a00</span>;
      }
    }
  }
}
</code></pre></div></details><hr><h2 id="murz" tabindex="-1"><a class="header-anchor" href="#murz"><span>Murz</span></a></h2><img src="`+u+'" alt="simple-theme-screenshot" style="display:block;margin-left:auto;margin-right:auto;">',11),g={class:"custom-container details"},f=s("summary",null,"expand theme.scss",-1),x={href:"https://github.com/Murzchnvok/rofi-collection",target:"_blank",rel:"noopener noreferrer"},w=a(`<div class="language-scss" data-ext="scss" data-title="scss"><pre class="language-scss"><code><span class="hljs-selector-class">.onagre</span> {
  <span class="hljs-attr">--exit-unfocused</span>: false;
  <span class="hljs-attribute">height</span>: <span class="hljs-number">250px</span>;
  <span class="hljs-attribute">width</span>: <span class="hljs-number">400px</span>;
  <span class="hljs-attr">--font-family</span>: <span class="hljs-string">&quot;Iosevka,Iosevka Nerd Font&quot;</span>;
  <span class="hljs-attribute">font-size</span>: <span class="hljs-number">12px</span>;
  <span class="hljs-attribute">background</span>: <span class="hljs-number">#18181b</span>;
  <span class="hljs-attribute">color</span>: <span class="hljs-number">#a0a0ab</span>;
  <span class="hljs-attribute">border-color</span>: <span class="hljs-number">#5d5e72</span>;
  <span class="hljs-attribute">border-width</span>: <span class="hljs-number">4px</span>;
  <span class="hljs-attribute">padding</span>: <span class="hljs-number">10px</span>;

  <span class="hljs-selector-class">.container</span> {
    <span class="hljs-selector-class">.search</span> {
      <span class="hljs-attr">--spacing</span>: <span class="hljs-number">1</span>;
      <span class="hljs-attribute">background</span>: <span class="hljs-number">#d8dee9</span>;
      <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">0</span>;
      <span class="hljs-attribute">color</span>: <span class="hljs-number">#18181b</span>;
      <span class="hljs-attribute">padding</span>: <span class="hljs-number">4px</span>;
      <span class="hljs-attr">--height</span>: fill-portion <span class="hljs-number">1</span>;
      <span class="hljs-selector-class">.plugin-hint</span> {
        <span class="hljs-attribute">font-size</span>: <span class="hljs-number">9px</span>;
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#d8dee9</span>;
        <span class="hljs-attribute">padding</span>: <span class="hljs-number">4px</span>;
        <span class="hljs-attribute">border-width</span>: <span class="hljs-number">2px</span>;
        <span class="hljs-attribute">border-color</span>: <span class="hljs-number">#5d5e72</span>;
        <span class="hljs-attribute">background</span>: <span class="hljs-number">#18181b</span>;
        <span class="hljs-attr">--align-x</span>: center;
        <span class="hljs-attr">--align-y</span>: center;
        <span class="hljs-attr">--width</span>: fill-portion <span class="hljs-number">2</span>;
        <span class="hljs-attr">--height</span>: fill;
      }

      <span class="hljs-selector-class">.input</span> {
        <span class="hljs-attr">--width</span>: fill-portion <span class="hljs-number">11</span>;
      }

    }

    <span class="hljs-selector-class">.rows</span> {
      <span class="hljs-attr">--height</span>: fill-portion <span class="hljs-number">6</span>;
      <span class="hljs-selector-class">.row-selected</span> {
        <span class="hljs-attribute">background</span>: <span class="hljs-number">#20212c</span>;
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#5d5e72</span>;
        <span class="hljs-attr">--spacing</span>: <span class="hljs-number">3px</span>;
      }
    }

    <span class="hljs-selector-class">.scrollable</span> {
      <span class="hljs-selector-class">.scroller</span> {
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#A0A0AB</span>;
        <span class="hljs-attribute">border-color</span>: <span class="hljs-number">#18181b</span>;
      }
    }
  }
}
</code></pre></div>`,1),y=a('<hr><h2 id="nord" tabindex="-1"><a class="header-anchor" href="#nord"><span>Nord</span></a></h2><img src="'+j+`" alt="simple-theme-screenshot" style="display:block;margin-left:auto;margin-right:auto;"><details class="custom-container details"><summary>expand theme.scss</summary><div class="language-scss" data-ext="scss" data-title="scss"><pre class="language-scss"><code><span class="hljs-selector-class">.onagre</span> {
  <span class="hljs-attr">--exit-unfocused</span>: false;
  <span class="hljs-attribute">height</span>: <span class="hljs-number">250px</span>;
  <span class="hljs-attribute">width</span>: <span class="hljs-number">400px</span>;
  <span class="hljs-attr">--icon-theme</span>: <span class="hljs-string">&quot;Papirus&quot;</span>;
  <span class="hljs-attr">--icon-size</span>: <span class="hljs-number">22px</span>;
  <span class="hljs-attr">--font-family</span>: <span class="hljs-string">&quot;Iosevka Nerd Font Mono&quot;</span>;
  <span class="hljs-attribute">font-size</span>: <span class="hljs-number">12px</span>;
  <span class="hljs-attribute">background</span>: <span class="hljs-number">#2E3440</span>;
  <span class="hljs-attribute">color</span>: <span class="hljs-number">#81a1c1</span>;
  <span class="hljs-attribute">border-color</span>: <span class="hljs-number">#2E3440</span>;
  <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">25%</span>;
  <span class="hljs-attribute">border-width</span>: <span class="hljs-number">4px</span>;
  <span class="hljs-attribute">padding</span>: <span class="hljs-number">10px</span>;

  <span class="hljs-selector-class">.container</span> {
    <span class="hljs-selector-class">.search</span> {
      <span class="hljs-attr">--spacing</span>: <span class="hljs-number">1</span>;
      <span class="hljs-attribute">background</span>: <span class="hljs-number">#3b4252</span>;
      <span class="hljs-attribute">color</span>: <span class="hljs-number">#d8dee9</span>;
      <span class="hljs-attribute">padding</span>: <span class="hljs-number">4px</span>;
      <span class="hljs-attr">--height</span>: fill-portion <span class="hljs-number">1</span>;
      <span class="hljs-selector-class">.plugin-hint</span> {
        <span class="hljs-attribute">font-size</span>: <span class="hljs-number">9px</span>;
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#bf616a</span>;
        <span class="hljs-attribute">padding</span>: <span class="hljs-number">4px</span>;
        <span class="hljs-attribute">border-width</span>: <span class="hljs-number">2px</span>;
        <span class="hljs-attribute">border-color</span>: <span class="hljs-number">#bf616a</span>;
        <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">5%</span>;
        <span class="hljs-attribute">background</span>: <span class="hljs-number">#4c566a</span>;
        <span class="hljs-attr">--align-x</span>: center;
        <span class="hljs-attr">--align-y</span>: center;
        <span class="hljs-attr">--width</span>: fill-portion <span class="hljs-number">2</span>;
        <span class="hljs-attr">--height</span>: fill;
      }

      <span class="hljs-selector-class">.input</span> {
        <span class="hljs-attr">--width</span>: fill-portion <span class="hljs-number">11</span>;
      }

    }

    <span class="hljs-selector-class">.rows</span> {
      <span class="hljs-attr">--height</span>: fill-portion <span class="hljs-number">5</span>;
      <span class="hljs-selector-class">.row-selected</span> {
        <span class="hljs-attribute">background</span>: <span class="hljs-number">#2E3440</span>;
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#ebcb8b</span>;
        <span class="hljs-attr">--spacing</span>: <span class="hljs-number">3px</span>;
        <span class="hljs-attr">--align-y</span>: center;
      }
    }

    <span class="hljs-selector-class">.scrollable</span> {
      <span class="hljs-selector-class">.scroller</span> {
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#4c566a</span>;
      }
    }
  }
}
</code></pre></div></details><hr><h2 id="not-adwaita" tabindex="-1"><a class="header-anchor" href="#not-adwaita"><span>Not-Adwaita</span></a></h2><img src="`+p+`" alt="simple-theme-screenshot" style="display:block;margin-left:auto;margin-right:auto;"><details class="custom-container details"><summary>expand theme.scss</summary><div class="language-scss" data-ext="scss" data-title="scss"><pre class="language-scss"><code><span class="hljs-selector-class">.onagre</span> {
  <span class="hljs-attribute">background</span>: <span class="hljs-number">#d6d6d6</span>;
  <span class="hljs-attribute">color</span>: <span class="hljs-number">#000000</span>;
  <span class="hljs-attr">--icon-theme</span>: <span class="hljs-string">&quot;Papirus&quot;</span>;
  <span class="hljs-attr">--font-family</span>: <span class="hljs-string">&quot;DejaVuSans&quot;</span>;
  <span class="hljs-attr">--icon-size</span>: <span class="hljs-number">24</span>;
  <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">8%</span>;
  <span class="hljs-attribute">border-color</span>: <span class="hljs-number">#d6d6d6</span>;
  <span class="hljs-attribute">border-width</span>: <span class="hljs-number">4px</span>;
  <span class="hljs-attribute">padding</span>: <span class="hljs-number">5px</span>;

  <span class="hljs-selector-class">.container</span> {
    <span class="hljs-selector-class">.rows</span> {
      <span class="hljs-attr">--height</span>: fill-portion <span class="hljs-number">6</span>;
      <span class="hljs-selector-class">.row</span> {
        <span class="hljs-attr">--width</span>: <span class="hljs-number">392</span>;

        <span class="hljs-selector-class">.icon</span> {
          <span class="hljs-attribute">padding-top</span>: <span class="hljs-number">4px</span>;
        }

        <span class="hljs-selector-class">.category-icon</span> {
          <span class="hljs-attribute">padding-left</span>: <span class="hljs-number">5px</span>;
          <span class="hljs-attr">--icon-size</span>: <span class="hljs-number">11</span>;
        }

        <span class="hljs-selector-class">.title</span> {
          <span class="hljs-attribute">font-size</span>: <span class="hljs-number">18px</span>;
        }

        <span class="hljs-selector-class">.description</span> {
          <span class="hljs-attribute">font-size</span>: <span class="hljs-number">12px</span>;
        }
      }

      <span class="hljs-selector-class">.row-selected</span> {
        <span class="hljs-attr">--width</span>: <span class="hljs-number">392</span>;
        <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">8%</span>;
        <span class="hljs-attribute">background</span>:  <span class="hljs-number">#c0c0c0</span>;

        <span class="hljs-selector-class">.icon</span> {
          <span class="hljs-attribute">padding-top</span>: <span class="hljs-number">4px</span>;
        }

        <span class="hljs-selector-class">.category-icon</span> {
          <span class="hljs-attribute">padding-left</span>: <span class="hljs-number">5px</span>;
          <span class="hljs-attr">--icon-size</span>: <span class="hljs-number">11</span>;
        }

        <span class="hljs-selector-class">.title</span> {
          <span class="hljs-attribute">font-size</span>: <span class="hljs-number">20px</span>;
        }

        <span class="hljs-selector-class">.description</span> {
          <span class="hljs-attribute">font-size</span>: <span class="hljs-number">12px</span>;
        }
      }
    }

    <span class="hljs-selector-class">.search</span> {
      <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">5%</span>;
      <span class="hljs-attribute">background</span>: <span class="hljs-number">#ffffff</span>;
      <span class="hljs-attr">--height</span>: fill-portion <span class="hljs-number">1</span>;
      <span class="hljs-attribute">padding</span>: <span class="hljs-number">4px</span>;
      <span class="hljs-selector-class">.input</span> {
        <span class="hljs-attribute">font-size</span>: <span class="hljs-number">20px</span>;
      }
    }

    <span class="hljs-selector-class">.scrollable</span> {
      <span class="hljs-attribute">width</span>: <span class="hljs-number">2px</span>;
      <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">5%</span>;
      <span class="hljs-attribute">background</span>: <span class="hljs-number">#c0c0c0</span>;
      <span class="hljs-selector-class">.scroller</span> {
        <span class="hljs-attribute">width</span>: <span class="hljs-number">4px</span>;
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#a1a1a1</span>;
      }
    }
  }
}
</code></pre></div></details><hr><h2 id="solarized" tabindex="-1"><a class="header-anchor" href="#solarized"><span>Solarized</span></a></h2><img src="`+b+`" alt="simple-theme-screenshot" style="display:block;margin-left:auto;margin-right:auto;"><details class="custom-container details"><summary>expand theme.scss</summary><div class="language-scss" data-ext="scss" data-title="scss"><pre class="language-scss"><code><span class="hljs-selector-class">.onagre</span> {
  <span class="hljs-attribute">background</span>: <span class="hljs-number">#fdf6e3</span>;
  <span class="hljs-attribute">color</span>: <span class="hljs-number">#657b83</span>;
  <span class="hljs-attr">--icon-theme</span>: <span class="hljs-string">&quot;Papirus&quot;</span>;
  <span class="hljs-attr">--font-family</span>: <span class="hljs-string">&quot;Monaco&quot;</span>;
  <span class="hljs-attr">--icon-size</span>: <span class="hljs-number">24</span>;
  <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">0</span>;
  <span class="hljs-attribute">border-color</span>: <span class="hljs-number">#a9b7c6</span>;
  <span class="hljs-attribute">border-width</span>: <span class="hljs-number">0</span>;
  <span class="hljs-attribute">height</span>: <span class="hljs-number">250px</span>;
  <span class="hljs-attribute">width</span>: <span class="hljs-number">440px</span>;

  <span class="hljs-selector-class">.container</span> {
    <span class="hljs-selector-class">.rows</span> {
      <span class="hljs-attr">--height</span>: fill-portion <span class="hljs-number">6</span>;
      <span class="hljs-selector-class">.row</span> {

        <span class="hljs-selector-class">.icon</span> {
          <span class="hljs-attribute">padding-top</span>: <span class="hljs-number">4px</span>;
        }

        <span class="hljs-selector-class">.title</span> {
          <span class="hljs-attribute">font-size</span>: <span class="hljs-number">18px</span>;
        }

        <span class="hljs-selector-class">.description</span> {
          <span class="hljs-attribute">font-size</span>: <span class="hljs-number">12px</span>;
        }
      }

      <span class="hljs-selector-class">.row-selected</span> {
        <span class="hljs-attr">--width</span>: <span class="hljs-number">435</span>;
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#268bd2</span>;

        <span class="hljs-selector-class">.icon</span> {
          <span class="hljs-attribute">padding-top</span>: <span class="hljs-number">4px</span>;
        }

        <span class="hljs-selector-class">.title</span> {
          <span class="hljs-attribute">font-size</span>: <span class="hljs-number">20px</span>;
        }

        <span class="hljs-selector-class">.description</span> {
          <span class="hljs-attribute">font-size</span>: <span class="hljs-number">12px</span>;
        }
      }
    }

    <span class="hljs-selector-class">.search</span> {
      <span class="hljs-attribute">background</span>: <span class="hljs-number">#fdf6e3</span>;
      <span class="hljs-attr">--height</span>: fill-portion <span class="hljs-number">1</span>;
      <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">0</span>;
      <span class="hljs-attribute">border-color</span>: <span class="hljs-number">#073642</span>;
      <span class="hljs-attribute">border-width</span>: <span class="hljs-number">3px</span>;
      <span class="hljs-attribute">padding</span>: <span class="hljs-number">4px</span>;
      <span class="hljs-selector-class">.input</span> {
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#002b36</span>;
        <span class="hljs-attr">--placeholder-color</span>: <span class="hljs-number">#657b83</span>;
        <span class="hljs-attr">--selection-color</span>: <span class="hljs-number">#2aa198</span>;
        <span class="hljs-attribute">font-size</span>: <span class="hljs-number">20px</span>;
        <span class="hljs-attr">--width</span>: fill-portion <span class="hljs-number">13</span>;
      }
      <span class="hljs-selector-class">.plugin-hint</span> {
        <span class="hljs-attribute">font-size</span>: <span class="hljs-number">11px</span>;
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#002b36</span>;
        <span class="hljs-attribute">padding</span>: <span class="hljs-number">6px</span>;
        <span class="hljs-attribute">border-color</span>: <span class="hljs-number">#859900</span>;
        <span class="hljs-attribute">background</span>: <span class="hljs-number">#fdf6e3</span>;
        <span class="hljs-attribute">border-width</span>: <span class="hljs-number">3px</span>;
        <span class="hljs-attr">--align-x</span>: center;
        <span class="hljs-attr">--align-y</span>: center;
        <span class="hljs-attr">--width</span>: fill-portion <span class="hljs-number">2</span>;
        <span class="hljs-attr">--height</span>: fill;
      }
    }

    <span class="hljs-selector-class">.scrollable</span> {
      <span class="hljs-attribute">width</span>: <span class="hljs-number">2px</span>;
      <span class="hljs-attribute">background</span>: <span class="hljs-number">#839496</span>;
      <span class="hljs-selector-class">.scroller</span> {
        <span class="hljs-attribute">border-radius</span>: <span class="hljs-number">0</span>;
        <span class="hljs-attribute">width</span>: <span class="hljs-number">2px</span>;
        <span class="hljs-attribute">color</span>: <span class="hljs-number">#268bd2</span>;
      }
    }
  }
}
</code></pre></div></details>`,12);function z(k,_){const l=e("ExternalLinkIcon");return c(),r("div",null,[m,s("details",g,[f,s("p",null,[n("credit to "),s("a",x,[n("murz"),h(l)])]),w]),y])}const N=t(d,[["render",z],["__file","gallery.html.vue"]]),E=JSON.parse('{"path":"/gallery.html","title":"Gallery","lang":"en-US","frontmatter":{},"headers":[{"level":2,"title":"Default theme","slug":"default-theme","link":"#default-theme","children":[]},{"level":2,"title":"Hollow","slug":"hollow","link":"#hollow","children":[]},{"level":2,"title":"Murz","slug":"murz","link":"#murz","children":[]},{"level":2,"title":"Nord","slug":"nord","link":"#nord","children":[]},{"level":2,"title":"Not-Adwaita","slug":"not-adwaita","link":"#not-adwaita","children":[]},{"level":2,"title":"Solarized","slug":"solarized","link":"#solarized","children":[]}],"git":{"updatedTime":1707918582000,"contributors":[{"name":"Paul Delafosse","email":"paul.delafosse@protonmail.com","commits":3}]},"filePathRelative":"gallery.md"}');export{N as comp,E as data};
