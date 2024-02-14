import { defaultTheme } from '@vuepress/theme-default'
import { defineUserConfig } from 'vuepress/cli'
import { viteBundler } from '@vuepress/bundler-vite'
import {highlightjsPlugin} from "./hljs/highlightjsPlugin";
import {copyCodePlugin} from "@vuepress/plugin-copy-code"

export default defineUserConfig({
  lang: 'en-US',
  description: 'A general purpose application launcher for X and wayland inspired by rofi/wofi and alfred, build with iced and pop-launcher.',
  title: " ",
  markdown: {
    code: {
      lineNumbers: false
    }
  },

  plugins: [
    highlightjsPlugin,
    copyCodePlugin({
      // options
    }),
  ],
  theme: defaultTheme({
    logo: '/onagre.png',
    repo: 'https://github.com/onagre-launcher/onagre',
    docsRepo: 'https://github.com/onagre-launcher/onagre',
    navbar: ['/', '/get-started', "/theming-reference", "/gallery"],
  }),
  bundler: viteBundler(),
  head: [
    ['link', { rel: 'icon', href: '/favicon.png' }],
    ['meta', {name: 'theme-color', content: '#ff9595'}],
    ['meta', {name: 'apple-mobile-web-app-capable', content: 'yes'}],
    ['meta', {name: 'apple-mobile-web-app-status-bar-style', content: 'black'}],
    ['meta', {property: 'og:title', content: 'Onagre'}],
    ['meta', {property: 'og:image', content: 'https://onagre-launcher.github.io/onagre/onagre.png'}],
    ['meta', {property: 'twitter:card', content: 'https://onagre-launcher.github.io/onagre/onagre.png'}],
    ['meta', {property: 'og:description', content: 'A general purpose application launcher for X and wayland  inspired by rofi/wofi and alfred, build with iced and pop-launcher.'}],
    ['meta', {property: 'og:width', content: '100'}],
  ],

})
