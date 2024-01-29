import { defaultTheme } from '@vuepress/theme-default'
import { defineUserConfig } from 'vuepress/cli'
import { viteBundler } from '@vuepress/bundler-vite'

export default defineUserConfig({
  lang: 'en-US',
  description: 'A general purpose application launcher for X and wayland inspired by rofi/wofi and alfred, build with iced and pop-launcher.',
  title: " ",
  theme: defaultTheme({
    logo: '/onagre.png',
    repo: 'https://github.com/onagre-launcher/onagre',
    docsRepo: 'https://github.com/onagre-launcher/onagre',
    navbar: ['/', '/get-started', "/theming-reference"],
  }),

  bundler: viteBundler(),
})
