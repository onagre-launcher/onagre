# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [1.1.1](https://github.com/onagre-launcher/onagre/compare/7c4285534d2e68853e1d79eb17dac7bef749b47d..1.1.1) - 2024-12-25
#### Bug Fixes
- fix onagre crashing on pop-launcher stderr - ([7c42855](https://github.com/onagre-launcher/onagre/commit/7c4285534d2e68853e1d79eb17dac7bef749b47d)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- thanks clippy - ([5431bf9](https://github.com/onagre-launcher/onagre/commit/5431bf9b7550b793174b9648985afb71d526a807)) - [@oknozor](https://github.com/oknozor)

- - -

## [1.1.0](https://github.com/onagre-launcher/onagre/compare/1.0.0..1.1.0) - 2024-04-18
#### Bug Fixes
- fix row height on iced 0.12 - ([0cd8a2a](https://github.com/onagre-launcher/onagre/commit/0cd8a2a5b6a10e304f7cecd85f2249a7856a9b46)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- revert manual bump for v1 and add docs deploy from main - ([faa1468](https://github.com/onagre-launcher/onagre/commit/faa1468888e8ac525b994c6ed2b39247527b89c1)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- Unnecessary spaces removed from the README.md - ([0136c54](https://github.com/onagre-launcher/onagre/commit/0136c54fb8e60c27e67d1f49e336f65df729d579)) - Doom-Git
- fix gh-pages domain - ([c57141f](https://github.com/onagre-launcher/onagre/commit/c57141f916bca8e2399b5a33b146ace973ac75cd)) - [@oknozor](https://github.com/oknozor)
- add gh page deployment custom domain - ([bb8e2be](https://github.com/onagre-launcher/onagre/commit/bb8e2bef513f139324d6e36c0d4d1e2b82b81546)) - [@oknozor](https://github.com/oknozor)
- update readme head - ([d9c2859](https://github.com/onagre-launcher/onagre/commit/d9c2859c25bc016e596f6f3a59547e90c9015f5f)) - [@oknozor](https://github.com/oknozor)
- update install instruction for stable release - ([ce87cfe](https://github.com/onagre-launcher/onagre/commit/ce87cfecc29f224bd49ed2ea3711163d57b78a64)) - [@oknozor](https://github.com/oknozor)
#### Features
- update iced to 0.12.1 - ([35248d4](https://github.com/onagre-launcher/onagre/commit/35248d444175d9893e6a63911212ad8ed5c73a60)) - [@oknozor](https://github.com/oknozor)
- switch to redb for cache performance - ([33655e3](https://github.com/onagre-launcher/onagre/commit/33655e33586818946377515945abb415ab105de5)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- clippy + fmt - ([2fecf42](https://github.com/onagre-launcher/onagre/commit/2fecf423494d331ccec354204363c40aa787233b)) - [@oknozor](https://github.com/oknozor)
- add logo - ([63da9a9](https://github.com/onagre-launcher/onagre/commit/63da9a97fb65fe5b96f8d1a1936edec3a6851cf1)) - [@oknozor](https://github.com/oknozor)

- - -

## [1.0.0](https://github.com/onagre-launcher/onagre/compare/4f7605b84e2bb0ba55b3caa9e3cf701b5bc03cb9..1.0.0) - 2024-02-14
#### Bug Fixes
- **(config)** make modes optional - ([77f0663](https://github.com/onagre-launcher/onagre/commit/77f0663aeaa2b2ca7d1e78593402b976eef0d46c)) - [@oknozor](https://github.com/oknozor)
- **(launcher)** remove useless placeholder item in web and terminal - ([70a7a62](https://github.com/onagre-launcher/onagre/commit/70a7a624a22e4daf5af5edd059bca3d78db4dc60)) - [@oknozor](https://github.com/oknozor)
- add some error context when pop-launcher is missing  - ([abb520c](https://github.com/onagre-launcher/onagre/commit/abb520cfe11264dbe0e9ef33699cfb7f5dfeea1d)) - [@oknozor](https://github.com/oknozor)
- use tracing instead of systemd log and fix default theme path - ([7fef4ce](https://github.com/onagre-launcher/onagre/commit/7fef4ce531a5fb948b24eb4541d812ce989305d7)) - [@oknozor](https://github.com/oknozor)
- fix typos in readme - ([a7628b1](https://github.com/onagre-launcher/onagre/commit/a7628b1d907e9e714197141c87adc2458c7bb7f3)) - Tristan
- style sheets not being applied to row, title and description widgets - ([632fc4c](https://github.com/onagre-launcher/onagre/commit/632fc4cd89aa0326917c3061c9dc32e089640716)) - Christian Friedow
- fix query from history for plugins - ([442585e](https://github.com/onagre-launcher/onagre/commit/442585eba8813934c9683afa897aee4261a6d7ad)) - [@oknozor](https://github.com/oknozor)
- Do not panic on font not found - ([a3efa08](https://github.com/onagre-launcher/onagre/commit/a3efa08ae86ad82c6845d579130a8340e6f32306)) - [@oknozor](https://github.com/oknozor)
- bump pop-launcher to 1.1.0 - ([a823ab3](https://github.com/onagre-launcher/onagre/commit/a823ab36f2a4146c333e4168b93c31485f9c7a72)) - [@oknozor](https://github.com/oknozor)
- make icon optional in desktop entries - ([b3e669c](https://github.com/onagre-launcher/onagre/commit/b3e669c89c0b9039637450fd65ca8a4117c45d3f)) - [@oknozor](https://github.com/oknozor)
- fix file autocompletion - ([997572a](https://github.com/onagre-launcher/onagre/commit/997572ac1f1e44d8e05ef0ca5894373b195770e2)) - [@oknozor](https://github.com/oknozor)
- snap scroll to top on input changed - ([9350acb](https://github.com/onagre-launcher/onagre/commit/9350acb3ec0dd93725a4ba275d5445b44cfb3867)) - [@oknozor](https://github.com/oknozor)
- switch back to iced master - ([1b9bac2](https://github.com/onagre-launcher/onagre/commit/1b9bac2c2839bfdaf167a750cd4c0e32a600470e)) - [@oknozor](https://github.com/oknozor)
- fix panic on empty matches - ([7e67262](https://github.com/onagre-launcher/onagre/commit/7e67262fafa938d8cbe8bbf76c9d286594727ec5)) - [@oknozor](https://github.com/oknozor)
- fix entry title spacing - ([16d921c](https://github.com/onagre-launcher/onagre/commit/16d921c193bd7f1edefa1c10444ad2ffdb937eb9)) - [@oknozor](https://github.com/oknozor)
- fix cache mess when enabling/disabling icons - ([ec81246](https://github.com/onagre-launcher/onagre/commit/ec812466c3142dd33f47cc1ec6a6b66bcbe085e1)) - [@oknozor](https://github.com/oknozor)
- fix default theme and custom menu loading - ([f261860](https://github.com/onagre-launcher/onagre/commit/f2618603aeecc89fb7b7be25a4e154b80661251f)) - [@oknozor](https://github.com/oknozor)
- fix duplication for custom entries - ([ae6173a](https://github.com/onagre-launcher/onagre/commit/ae6173aae4896f199fe458603a52cb9ebf0fdd19)) - [@oknozor](https://github.com/oknozor)
- fix duplication for desktop entries - ([7b02f1e](https://github.com/onagre-launcher/onagre/commit/7b02f1ede29cfae17f1cb57e7aa26c920690e4ea)) - [@oknozor](https://github.com/oknozor)
- fix default theme - ([9178de4](https://github.com/onagre-launcher/onagre/commit/9178de49b9a0f69d27bfbf8ddcbeb23d4e8e15f8)) - [@oknozor](https://github.com/oknozor)
- fix subsctract owerflow on index + key down when no entries are loaded - ([6304bf8](https://github.com/onagre-launcher/onagre/commit/6304bf8e6598fac1b317b3b7aa07a261ad6464fa)) - [@oknozor](https://github.com/oknozor)
- add wgpu patch for reansparency - ([e521654](https://github.com/onagre-launcher/onagre/commit/e521654ac171cb80946b170baa2105c628a036fa)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- fix gh pages - ([c1faf42](https://github.com/onagre-launcher/onagre/commit/c1faf428cd9d1ebbdcd7f242d65bb57c9abc5400)) - [@oknozor](https://github.com/oknozor)
- Fix release output - ([6e1bdd3](https://github.com/onagre-launcher/onagre/commit/6e1bdd31256b4236066404ae8e5ebb8fcea3276f)) - [@oknozor](https://github.com/oknozor)
- remove cargo publish - ([9fec426](https://github.com/onagre-launcher/onagre/commit/9fec426182e1a11019f43435379cacdc950d19d5)) - [@oknozor](https://github.com/oknozor)
- remove useless action checkout token - ([4b67d2a](https://github.com/onagre-launcher/onagre/commit/4b67d2a32ab9a154db5cf180ba2e42cb65071f5f)) - [@oknozor](https://github.com/oknozor)
- change gh page vuepress action - ([f47f09b](https://github.com/onagre-launcher/onagre/commit/f47f09b4f840169df59b4e055e8cf2aca82bd29c)) - [@oknozor](https://github.com/oknozor)
- add musl dependencies to cross image - ([29bdc3a](https://github.com/onagre-launcher/onagre/commit/29bdc3ace677bbf18957c178239952bc59e26aa3)) - [@oknozor](https://github.com/oknozor)
- update Cross config - ([8924820](https://github.com/onagre-launcher/onagre/commit/89248201195aee048cd6c1d6633494fcd742e576)) - [@oknozor](https://github.com/oknozor)
- update release workflow - ([75c73ac](https://github.com/onagre-launcher/onagre/commit/75c73ac7252821579533417f5dcd7729a33893e1)) - [@oknozor](https://github.com/oknozor)
- update cog release - ([ad4eebc](https://github.com/onagre-launcher/onagre/commit/ad4eebc368adeb1faf21fbf90f634d46cd3aaf4a)) - [@oknozor](https://github.com/oknozor)
- add continuous delivery action - ([fe03876](https://github.com/onagre-launcher/onagre/commit/fe038760dac66f5914e2f54d775a8ba731a77233)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- **(web)** add hollow theme to the gallery - ([37a095f](https://github.com/onagre-launcher/onagre/commit/37a095fdc3de8199b70f066c8a3033462de93fb5)) - [@oknozor](https://github.com/oknozor)
- fix broken readme img links - ([af68446](https://github.com/onagre-launcher/onagre/commit/af684465032a7b5f4ac5425f0e8764890e1ac899)) - [@oknozor](https://github.com/oknozor)
- add gallery and plugin docs - ([63f1843](https://github.com/onagre-launcher/onagre/commit/63f184312a24f607b7f6b265049914fbeaed73f7)) - [@oknozor](https://github.com/oknozor)
- add getting started guide - ([97ea7fb](https://github.com/onagre-launcher/onagre/commit/97ea7fb8252a215e6a56915bcb576d671f476a2e)) - [@oknozor](https://github.com/oknozor)
- add vuepress website - ([14771e6](https://github.com/onagre-launcher/onagre/commit/14771e61c44eb0afe90ecc7985c62fb2f2260cc5)) - [@oknozor](https://github.com/oknozor)
- add locked flag to manual installation instruction - ([c8410a4](https://github.com/onagre-launcher/onagre/commit/c8410a4da55a44332aa39cd0d3dd9ce558e3313b)) - [@oknozor](https://github.com/oknozor)
- Update README.md to include packaging status (#59) - ([b7185f2](https://github.com/onagre-launcher/onagre/commit/b7185f2bdfed5a2e96df1dbff460432e0de10019)) - Merlin
- add logo - ([8d4ffe9](https://github.com/onagre-launcher/onagre/commit/8d4ffe9f7f38cb2d1eb3023a128856c209100cf1)) - [@oknozor](https://github.com/oknozor)
- add more theme screenshots and examples - ([17569c9](https://github.com/onagre-launcher/onagre/commit/17569c945df310fda25ef6f72333151d29e92cec)) - [@oknozor](https://github.com/oknozor)
- add a link to the wiki theme section - ([f48be06](https://github.com/onagre-launcher/onagre/commit/f48be066f2c58b29c492df6af6858a0c3065625a)) - [@oknozor](https://github.com/oknozor)
- set file name for mp4 demo - ([e90e706](https://github.com/onagre-launcher/onagre/commit/e90e70672c363f6de444ea747171ebae05800fc9)) - [@oknozor](https://github.com/oknozor)
- add mp4 demo - ([67b26c1](https://github.com/onagre-launcher/onagre/commit/67b26c11b79cad7f24e9dabb0e529092e52622c2)) - [@oknozor](https://github.com/oknozor)
- add default theme to the galery section - ([6db9525](https://github.com/onagre-launcher/onagre/commit/6db9525c60c7e564a4dace5c5a983d2a5b2a67b1)) - [@oknozor](https://github.com/oknozor)
- update readme - ([ba541ba](https://github.com/onagre-launcher/onagre/commit/ba541ba4947a9e5ebf43233c09c18e680dd07705)) - [@oknozor](https://github.com/oknozor)
- update README.md - ([86e19bf](https://github.com/onagre-launcher/onagre/commit/86e19bfb632234003a826504ea178180083bcd0f)) - [@oknozor](https://github.com/oknozor)
- update README - ([45ec224](https://github.com/onagre-launcher/onagre/commit/45ec2246158c8d57a50b6f772912f548383ae7d5)) - [@oknozor](https://github.com/oknozor)
- add theme examples - ([2407633](https://github.com/onagre-launcher/onagre/commit/2407633d280a90fbed71db1c67395e881dc07ade)) - [@oknozor](https://github.com/oknozor)
- transparancy androunded corner are now working with iced latest - ([3c495cf](https://github.com/onagre-launcher/onagre/commit/3c495cf2754320c2446221881c25076c1baa9747)) - [@oknozor](https://github.com/oknozor)
- update README - ([b819ef9](https://github.com/onagre-launcher/onagre/commit/b819ef9a293141e348bd8204f17c7431a6dd36ae)) - [@oknozor](https://github.com/oknozor)
- add link to AUR repo - ([b9380f6](https://github.com/onagre-launcher/onagre/commit/b9380f63a10e59e838e02ae19529a3fa47276f48)) - Jason Nader
- Fix typo - ([ea9f4eb](https://github.com/onagre-launcher/onagre/commit/ea9f4ebb9356ba4a5898d797a74d0c230798d491)) - Jason Nader
- fix readme titles - ([69380c8](https://github.com/onagre-launcher/onagre/commit/69380c87813da006363db5fe9f98bd913f7f8d34)) - [@oknozor](https://github.com/oknozor)
- add instructions on how to hide menu - ([32f06a8](https://github.com/onagre-launcher/onagre/commit/32f06a8c69444716ad540b46bc3ef9ba6626cf3d)) - [@oknozor](https://github.com/oknozor)
- update README - ([69d3467](https://github.com/onagre-launcher/onagre/commit/69d346795c6a3eb26027200dfbdfed6edd25d11b)) - [@oknozor](https://github.com/oknozor)
- update README and default theme - ([3058605](https://github.com/onagre-launcher/onagre/commit/3058605e5f7b4707bb479dc26b5a496d1b0fd4aa)) - [@oknozor](https://github.com/oknozor)
- add some comments explaining icon lookup - ([bcc8377](https://github.com/onagre-launcher/onagre/commit/bcc83774f67a94eb76d8f57573eaa887182f71b6)) - [@oknozor](https://github.com/oknozor)
- add README and screenshot - ([3860069](https://github.com/onagre-launcher/onagre/commit/386006995d55b829bdbc8ac6c3531370ab1b99b4)) - [@oknozor](https://github.com/oknozor)
- add config menu example - ([30fa576](https://github.com/onagre-launcher/onagre/commit/30fa576a2cde1627ff013d715cbf5a2eac5dedfb)) - [@oknozor](https://github.com/oknozor)
#### Features
- **(style)** add custom font to styling - ([6aa9951](https://github.com/onagre-launcher/onagre/commit/6aa9951d994b6c9282e7babeba75aad104390194)) - [@oknozor](https://github.com/oknozor)
- update deps and add copy code plugin - ([657363c](https://github.com/onagre-launcher/onagre/commit/657363cc7f65e524e2f691bc2985fdf8af5971ff)) - [@oknozor](https://github.com/oknozor)
- make row clickable - ([9adb72b](https://github.com/onagre-launcher/onagre/commit/9adb72b4fde247d419fba0a660f28145f1ca8705)) - [@oknozor](https://github.com/oknozor)
- add theme scaling - ([8d496ac](https://github.com/onagre-launcher/onagre/commit/8d496ac1dea49e502393003df11a13b4f25e29ec)) - [@oknozor](https://github.com/oknozor)
- launch onagre with mode as cli parameter - ([feb3249](https://github.com/onagre-launcher/onagre/commit/feb3249445b0c6876ab06b285ede7c235f9360c8)) - Juan Pablo
- log to systemd - ([d164857](https://github.com/onagre-launcher/onagre/commit/d1648579d5ad8815ceb4dde1c5b8274b9b3592bb)) - [@oknozor](https://github.com/oknozor)
- get category icon for db entries - ([2a2ddcd](https://github.com/onagre-launcher/onagre/commit/2a2ddcd4e19fa8d785c9aa3829dd8642522fcfa2)) - [@oknozor](https://github.com/oknozor)
- use user theme for symbolic icons - ([93df4bc](https://github.com/onagre-launcher/onagre/commit/93df4bcedb732c056f09905446de7ae08c0f6d37)) - [@oknozor](https://github.com/oknozor)
- add more theme examples - ([bf57a14](https://github.com/onagre-launcher/onagre/commit/bf57a14df5ada08d0629b9692b0afcf50e4d9281)) - [@oknozor](https://github.com/oknozor)
- add row spacing and some theme examples - ([60d7aa5](https://github.com/onagre-launcher/onagre/commit/60d7aa54f57f93985281a084ffe847841f33c1ab)) - [@oknozor](https://github.com/oknozor)
- add category icons - ([9062665](https://github.com/onagre-launcher/onagre/commit/90626652d6046d827f0bc6e627dd6884899ebc22)) - [@oknozor](https://github.com/oknozor)
- add fallback icon and fonts/icons licences - ([3803734](https://github.com/onagre-launcher/onagre/commit/380373458f28eeec5372dcc1407dde966e327f0c)) - [@oknozor](https://github.com/oknozor)
- add base theme for config override - ([d86480a](https://github.com/onagre-launcher/onagre/commit/d86480a38f9362498aed82a5c17340a786d7b5e0)) - [@oknozor](https://github.com/oknozor)
- add width and height to .rows stylesheet - ([6d4eb04](https://github.com/onagre-launcher/onagre/commit/6d4eb04b05664f123520e1b52feeab4f300f4c54)) - [@oknozor](https://github.com/oknozor)
- embed jetbrains mono as a default font - ([4207ff9](https://github.com/onagre-launcher/onagre/commit/4207ff96ffea35d7f299ac685331c74d74b5a884)) - [@oknozor](https://github.com/oknozor)
- add default theme - ([cb4e2df](https://github.com/onagre-launcher/onagre/commit/cb4e2df36e078dc70acbee9018119033bb9d84de)) - [@oknozor](https://github.com/oknozor)
- add inheritance for config properties - ([5466067](https://github.com/onagre-launcher/onagre/commit/54660673206486377be9531b2c7ed00ed9100b93)) - [@oknozor](https://github.com/oknozor)
- add 'font-size' for title and description - ([9ade1f6](https://github.com/onagre-launcher/onagre/commit/9ade1f682dbc2cede9f32fe1bb7bccd08dc82829)) - [@oknozor](https://github.com/oknozor)
- allow to deactivate plugin-hint via configuration - ([caaca54](https://github.com/onagre-launcher/onagre/commit/caaca54c06a238a1d0a210cfa622d78fd0401a9b)) - [@oknozor](https://github.com/oknozor)
- display plugin hint conditionally - ([3e67207](https://github.com/onagre-launcher/onagre/commit/3e67207a8276fe85a9d13468c639e4c072b7df22)) - [@oknozor](https://github.com/oknozor)
- implement css like stylesheet - ([9952f47](https://github.com/onagre-launcher/onagre/commit/9952f47bc3893655c092f7b0da8c4eaee73bf462)) - [@oknozor](https://github.com/oknozor)
- add desktop entry description - ([3cdabee](https://github.com/onagre-launcher/onagre/commit/3cdabee60a90fd8bb31f12314baa0d8e4b193acf)) - [@oknozor](https://github.com/oknozor)
- add base stylesheet for plugin hint - ([13b0774](https://github.com/onagre-launcher/onagre/commit/13b0774eea403f79be19c394a2e8737cb7c27432)) - [@oknozor](https://github.com/oknozor)
- switch to pop-launcher-toolkit - ([2e1e21e](https://github.com/onagre-launcher/onagre/commit/2e1e21e6fc6ea72fa3a3d53a1fcd49fb9fadfee1)) - [@oknozor](https://github.com/oknozor)
- add modifier hilight - ([7ed00ae](https://github.com/onagre-launcher/onagre/commit/7ed00aeecf0e8f561ba6da0ab1583bfde4405fa1)) - [@oknozor](https://github.com/oknozor)
- close onagre  on focus lost - ([1d6011f](https://github.com/onagre-launcher/onagre/commit/1d6011f3880a43c2073ce2001896fbf32f02f3ff)) - [@oknozor](https://github.com/oknozor)
- remove custom terminal entries and consequently onagre config file - ([9cf953a](https://github.com/onagre-launcher/onagre/commit/9cf953a8f9ce33acd91ef5434ea0a719b2d7f413)) - [@oknozor](https://github.com/oknozor)
- add wayland app_id - ([ad0c720](https://github.com/onagre-launcher/onagre/commit/ad0c720d7ca0ed9440f2a57a7a78a26ee9576a01)) - [@oknozor](https://github.com/oknozor)
- implement favicon display for web entry - ([8e305f2](https://github.com/onagre-launcher/onagre/commit/8e305f25fc1a7d00d0b9d77dbf41000a0e7dc847)) - [@oknozor](https://github.com/oknozor)
- web search history and fix line selection - ([098bc41](https://github.com/onagre-launcher/onagre/commit/098bc41113b621d7301143968168183c41186766)) - [@oknozor](https://github.com/oknozor)
- refactor modes to implement terminal history - ([b4cdf9c](https://github.com/onagre-launcher/onagre/commit/b4cdf9ca7a259248533d5d32a59bce19be892031)) - [@oknozor](https://github.com/oknozor)
- implement desktop entry history - ([28813b6](https://github.com/onagre-launcher/onagre/commit/28813b648f60e406141acf8ee7888cdb73b1f519)) - [@oknozor](https://github.com/oknozor)
- add template modes - ([1e2256c](https://github.com/onagre-launcher/onagre/commit/1e2256cfbd3abc61c2f51d52ea0688b7d97965a9)) - [@oknozor](https://github.com/oknozor)
- set default modes to all - ([54dc9bb](https://github.com/onagre-launcher/onagre/commit/54dc9bbdd0f02be37fba636ec3cf6ebb2ad184ff)) - [@oknozor](https://github.com/oknozor)
- get modes from user input - ([e0e58d3](https://github.com/onagre-launcher/onagre/commit/e0e58d3f874fbe8a810e9b13205de0a0fd4c346a)) - [@oknozor](https://github.com/oknozor)
- display config error to stderr - ([32574d0](https://github.com/onagre-launcher/onagre/commit/32574d07c98b901481c06f8df78487e36268b96b)) - [@oknozor](https://github.com/oknozor)
- add naive duplication filter for custom modes - ([83fd678](https://github.com/onagre-launcher/onagre/commit/83fd67871ef5b6ae6873bae3d6ce9bd9226a5de2)) - [@oknozor](https://github.com/oknozor)
- sort match by score + weight to get finer results - ([9e84f49](https://github.com/onagre-launcher/onagre/commit/9e84f49e5a529e9144521f13d833a4400324f7bc)) - [@oknozor](https://github.com/oknozor)
- implement cache - ([5861990](https://github.com/onagre-launcher/onagre/commit/58619904dd3f224d86a19b0c125b2ace06f377dc)) - [@oknozor](https://github.com/oknozor)
- match desktop entry on keywords and app name - ([4684cec](https://github.com/onagre-launcher/onagre/commit/4684cecdd7517550ceaaa98b96e9ec80ed871e7c)) - [@oknozor](https://github.com/oknozor)
- filter duplicate desktop entries - ([3dbe6f5](https://github.com/onagre-launcher/onagre/commit/3dbe6f51301ac1246bba032475aa8b05fefbcf29)) - [@oknozor](https://github.com/oknozor)
- add desktop icons - ([3d7b76f](https://github.com/onagre-launcher/onagre/commit/3d7b76f56b7e8ee0f0a6a68efdbf2b05d6993edd)) - [@oknozor](https://github.com/oknozor)
- base icon search implementation - ([18e03db](https://github.com/onagre-launcher/onagre/commit/18e03db9547cbd1d4bb889f63c6945307adb1c21)) - [@oknozor](https://github.com/oknozor)
- get custom modes from config - ([a47f8a8](https://github.com/onagre-launcher/onagre/commit/a47f8a88cbf0007b7d85f4e555b77ee0c7ea3ba6)) - [@oknozor](https://github.com/oknozor)
- expose paggind in theme config - ([99ef484](https://github.com/onagre-launcher/onagre/commit/99ef484bcadfa8c9665705128d9a5f467f6e5c0b)) - [@oknozor](https://github.com/oknozor)
- add custom lenght type for config deserializtion - ([a5a62a5](https://github.com/onagre-launcher/onagre/commit/a5a62a5e0dcdc33980b654cdeca101a40361715f)) - [@oknozor](https://github.com/oknozor)
- add optional opacity to onagre color - ([3899d70](https://github.com/onagre-launcher/onagre/commit/3899d70eb382dbdc7d5d5fdcc83a02be46f011d9)) - [@oknozor](https://github.com/oknozor)
- custom commands - ([fa502b4](https://github.com/onagre-launcher/onagre/commit/fa502b4a5bce58398bc0e386cb67279af5d4e765)) - [@oknozor](https://github.com/oknozor)
- theme from config file - ([60cd37f](https://github.com/onagre-launcher/onagre/commit/60cd37f002fa1e4da7c87b3fc1b1b2e31dcb69ae)) - [@oknozor](https://github.com/oknozor)
- desktop application dummy impl - ([0aae9e1](https://github.com/onagre-launcher/onagre/commit/0aae9e1075f310234b0354994f258a0927f3e967)) - [@oknozor](https://github.com/oknozor)
- theming base - ([c0327a4](https://github.com/onagre-launcher/onagre/commit/c0327a477e59f15777f14dbabed3795c641b9b1f)) - [@oknozor](https://github.com/oknozor)
- use smart pointers to avoid cloning search matches - ([d4566d2](https://github.com/onagre-launcher/onagre/commit/d4566d2aaeb0be0b6750211e1098565f14726905)) - [@oknozor](https://github.com/oknozor)
- add asynchronous file walker - ([0c558c8](https://github.com/onagre-launcher/onagre/commit/0c558c8a5c24cb940a84b428e9571ffe40364ba9)) - [@oknozor](https://github.com/oknozor)
- remove sway exec on command launch - ([cc81099](https://github.com/onagre-launcher/onagre/commit/cc8109926a374b81e622b4edef1d6126e4406d1e)) - [@oknozor](https://github.com/oknozor)
- add default styling - ([ef0db9e](https://github.com/onagre-launcher/onagre/commit/ef0db9e2cff780e84126594d5840fec7deeccf2a)) - [@oknozor](https://github.com/oknozor)
- launch app - ([0c4c4ec](https://github.com/onagre-launcher/onagre/commit/0c4c4ec28c1b54654b721c629df1fd8880967696)) - [@oknozor](https://github.com/oknozor)
- search and select - ([58adab4](https://github.com/onagre-launcher/onagre/commit/58adab48ddba7a667de5659c70b083bf6ccfe351)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- bump cargo lock - ([297d8d1](https://github.com/onagre-launcher/onagre/commit/297d8d19d8c817b10fee3194b447f10b524418f0)) - [@oknozor](https://github.com/oknozor)
- fix launcher dependencies - ([b277745](https://github.com/onagre-launcher/onagre/commit/b277745be5c7edb0f9b3d6ca937359336ece326a)) - [@oknozor](https://github.com/oknozor)
- manual bump for v1 - ([bfc625a](https://github.com/onagre-launcher/onagre/commit/bfc625a34c39be89004c3a06fb3b6bd6344ef5f2)) - [@oknozor](https://github.com/oknozor)
- update dependencies - ([c9b0c5c](https://github.com/onagre-launcher/onagre/commit/c9b0c5cf9b3f80fa9e33453a6ef12edebf4cc78d)) - [@oknozor](https://github.com/oknozor)
- remaplace structopts with clap - ([41d5119](https://github.com/onagre-launcher/onagre/commit/41d51191214a81a0afc6c9a91fd6bcc8715383cf)) - [@oknozor](https://github.com/oknozor)
- bump freedesktop-icons to 0.2.5 - ([e7239b0](https://github.com/onagre-launcher/onagre/commit/e7239b05c683d3e2613c2212185bd6862b29f188)) - [@oknozor](https://github.com/oknozor)
- fmt, clippy, cargo update - ([2dc6c21](https://github.com/onagre-launcher/onagre/commit/2dc6c21d51bf95479202b0502e5694e05ada3285)) - [@oknozor](https://github.com/oknozor)
- update cog.toml - ([5adac8e](https://github.com/onagre-launcher/onagre/commit/5adac8eaae52c01d9c5d3117c95de45a1d8a4d21)) - [@oknozor](https://github.com/oknozor)
- remove unused bench - ([ae977b6](https://github.com/onagre-launcher/onagre/commit/ae977b64b541b1c6695be356843d2c8d67e3c6bd)) - [@oknozor](https://github.com/oknozor)
- migrate to pop launcher fork - ([8233b8a](https://github.com/onagre-launcher/onagre/commit/8233b8a50d0a1c471950fac970729855862bd839)) - [@oknozor](https://github.com/oknozor)
- update dependencies - ([58bc255](https://github.com/onagre-launcher/onagre/commit/58bc255f77dccbfef7232f975bff4a8f34e42993)) - [@oknozor](https://github.com/oknozor)
- update dependencies - ([c044ccf](https://github.com/onagre-launcher/onagre/commit/c044ccf42d4d5048cd9abb49cf8b64229eb84a93)) - [@oknozor](https://github.com/oknozor)
- add continuous integration action - ([ee4f882](https://github.com/onagre-launcher/onagre/commit/ee4f882d5c9f26553af5ca9601ac45bf249b33c2)) - [@oknozor](https://github.com/oknozor)
- fmt + clippy - ([177d189](https://github.com/onagre-launcher/onagre/commit/177d189993d2d909f09554497d01d43817b9ada2)) - [@oknozor](https://github.com/oknozor)
- update to iced latest - ([82b3cf9](https://github.com/onagre-launcher/onagre/commit/82b3cf954ea6f785d7bb0fbc1692ae77e2ce8a14)) - [@oknozor](https://github.com/oknozor)
- add cargo metadata - ([e19f2ac](https://github.com/onagre-launcher/onagre/commit/e19f2ac900c7bc8546d6c632e7ae1a7480fb60da)) - [@oknozor](https://github.com/oknozor)
- lock dependencies and add build instruction - ([b4ee3cc](https://github.com/onagre-launcher/onagre/commit/b4ee3cc53c2279813adaa3f357583318cbf3c26e)) - [@oknozor](https://github.com/oknozor)
- clippy lints - ([6417641](https://github.com/onagre-launcher/onagre/commit/6417641e6a6eb384c6271cc4dd8da73703b4d8ab)) - [@oknozor](https://github.com/oknozor)
- remove unused deps - ([e6a0e1a](https://github.com/onagre-launcher/onagre/commit/e6a0e1aeab6e3b4ce4543b6ca3afc71077272276)) - [@oknozor](https://github.com/oknozor)
- add a temporary debug 'ugly' theme - ([1de4ec1](https://github.com/onagre-launcher/onagre/commit/1de4ec192729ec7421ad66ae1472afcb3e4b5eed)) - [@oknozor](https://github.com/oknozor)
- fix clippy lints - ([fbb1237](https://github.com/onagre-launcher/onagre/commit/fbb12373722e0daaff235fc3f86784a7187f76db)) - [@oknozor](https://github.com/oknozor)
- update cog.toml to latest format - ([8b37d47](https://github.com/onagre-launcher/onagre/commit/8b37d4733e8743efc6824d78003612d064d16c6b)) - [@oknozor](https://github.com/oknozor)
- add github sponsor - ([a3dccbc](https://github.com/onagre-launcher/onagre/commit/a3dccbcbb2f26b7ebdc3033c7bb58c30bebcad73)) - [@oknozor](https://github.com/oknozor)
- fix debug log - ([a806b97](https://github.com/onagre-launcher/onagre/commit/a806b97b4628ca960d90405776d157bd1daac9bf)) - [@oknozor](https://github.com/oknozor)
- fmt all - ([f481a2d](https://github.com/onagre-launcher/onagre/commit/f481a2dc5e72100741b1f1c1042b7c7b80dba8b9)) - [@oknozor](https://github.com/oknozor)
- cleanup debug - ([d8d1b9d](https://github.com/onagre-launcher/onagre/commit/d8d1b9d6d205b6437a5742a70fdf5bbee6b69583)) - [@oknozor](https://github.com/oknozor)
- reduce dependencies introduced by the clap crateThis disables coloured error messages and Unicode char support in help messages,which saves a couple of dependencies. Since this is a GUI application then itshould be acceptable. - ([933d36d](https://github.com/onagre-launcher/onagre/commit/933d36d39f0ad25e8fc00c6d5e0510157db0b3f8)) - Jason Nader
- cargo update - ([15695a5](https://github.com/onagre-launcher/onagre/commit/15695a590e26156bfb5f8edd0134844587b266cd)) - [@oknozor](https://github.com/oknozor)
- remove sway specific config - ([5e183fb](https://github.com/onagre-launcher/onagre/commit/5e183fba71e51cb1e6d37bed522b398429bf1a71)) - [@oknozor](https://github.com/oknozor)
- fix clippy lints - ([68c26ed](https://github.com/onagre-launcher/onagre/commit/68c26ed43579f67ec74ad93fd52c4b38cec5bfe6)) - [@oknozor](https://github.com/oknozor)
- update iced and fix build - ([217e815](https://github.com/onagre-launcher/onagre/commit/217e8156b2d37bf8653d15f4bcfa7765173ae8a9)) - [@oknozor](https://github.com/oknozor)
- dump settings in debug mode - ([c052307](https://github.com/onagre-launcher/onagre/commit/c052307d5692d78b9cd1fc9ad6b914065f5bc835)) - [@oknozor](https://github.com/oknozor)
- add debug info for icon builder - ([c23c580](https://github.com/onagre-launcher/onagre/commit/c23c580b58b7b52b9701ef877e4a7e0babce2e85)) - [@oknozor](https://github.com/oknozor)
- update issue templates - ([ad4993a](https://github.com/onagre-launcher/onagre/commit/ad4993a8d76f14d4867c6f21e4026f4f607a4454)) - [@oknozor](https://github.com/oknozor)
- fmt all - ([f946c95](https://github.com/onagre-launcher/onagre/commit/f946c95f1f2042a213e1feecdacd44a429ce4b52)) - [@oknozor](https://github.com/oknozor)
- add MIT license - ([6cf30c5](https://github.com/onagre-launcher/onagre/commit/6cf30c5222638db251c70fa49ee0f2cabbc49ff4)) - [@oknozor](https://github.com/oknozor)
- remove unused state field - ([3253f8f](https://github.com/onagre-launcher/onagre/commit/3253f8fb17ea4960ce9769a01a52d83824e6d689)) - [@oknozor](https://github.com/oknozor)
- remove old todos - ([94b0721](https://github.com/onagre-launcher/onagre/commit/94b0721052cbae38796d44467e4a168b3b28b872)) - [@oknozor](https://github.com/oknozor)
- remove useless placeholder - ([eb30431](https://github.com/onagre-launcher/onagre/commit/eb30431dfa9c11c2032cd32944dac9a81aa32781)) - [@oknozor](https://github.com/oknozor)
- remove unused backspace event - ([95747bc](https://github.com/onagre-launcher/onagre/commit/95747bce557116ad6a6904ac0d7717eb744e168b)) - [@oknozor](https://github.com/oknozor)
- fmt all - ([7e991f6](https://github.com/onagre-launcher/onagre/commit/7e991f6592000aaa454e975736dd70f629614504)) - [@oknozor](https://github.com/oknozor)
- remove unused imports - ([8aa1747](https://github.com/onagre-launcher/onagre/commit/8aa174769958e72482a9eeffb823c9e74b4bebaf)) - [@oknozor](https://github.com/oknozor)
- remove glow backent cargo feaature - ([bc0cd5a](https://github.com/onagre-launcher/onagre/commit/bc0cd5a3f8a9cfbab24fe3fa2f954df076535f40)) - [@oknozor](https://github.com/oknozor)
- fmt & clippy - ([95ad096](https://github.com/onagre-launcher/onagre/commit/95ad0963cad57f46a67cf4ed1866490fc70274e5)) - [@oknozor](https://github.com/oknozor)
- mode experiment with rayon - ([8a85eeb](https://github.com/onagre-launcher/onagre/commit/8a85eebaf1197afe12ccbb408f97568260f672c3)) - [@oknozor](https://github.com/oknozor)
#### Performance Improvements
- **(icon)** improve icon finder perf - ([fbe21e5](https://github.com/onagre-launcher/onagre/commit/fbe21e500d8c60634020304137ab606b9b07ff32)) - [@oknozor](https://github.com/oknozor)
- add some benchmark and cargo opt profile - ([c7650cf](https://github.com/onagre-launcher/onagre/commit/c7650cf5dc597036bc25236344741a1c885ffc3f)) - [@oknozor](https://github.com/oknozor)
- allocate entries vec with a capacity of 256 - ([a9d380b](https://github.com/onagre-launcher/onagre/commit/a9d380b76c7bac61a50edc4f27ea6ba608b0c043)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- use pop-launcher-toolkit to access web plugin config - ([73e0c7a](https://github.com/onagre-launcher/onagre/commit/73e0c7a857ea712d64fa7ef5c819f1af9df6e143)) - [@oknozor](https://github.com/oknozor)
- reorganize modules again - ([f8da4f0](https://github.com/onagre-launcher/onagre/commit/f8da4f011eba49041d0b79ff5fe0d46fac4bb619)) - [@oknozor](https://github.com/oknozor)
- use Cow<str> for database entries - ([619e8d5](https://github.com/onagre-launcher/onagre/commit/619e8d5c518674f18f638fbcc3cdb33d140ab110)) - [@oknozor](https://github.com/oknozor)
- back to pop-os upstream, switch to freedesktop-icons for icon lookup and add an exit_unfocused flag - ([5bbbc49](https://github.com/onagre-launcher/onagre/commit/5bbbc49f39934c4e5883e8ec9b036d6cb210b5c7)) - [@oknozor](https://github.com/oknozor)
- reorganize everything, update pop_launcher and switch to tokio runtimme ... - ([492bd67](https://github.com/onagre-launcher/onagre/commit/492bd67f43c10299a5ee37af6157654edda8d200)) - [@oknozor](https://github.com/oknozor)
- replace lazy_static with once_cell - ([2a7db18](https://github.com/onagre-launcher/onagre/commit/2a7db181f61cf9a2f24fbd227c97f378beee7563)) - [@oknozor](https://github.com/oknozor)
- remove useless allocation for terminal and web mode entries - ([ca4fe2f](https://github.com/onagre-launcher/onagre/commit/ca4fe2fc61d5757b4247ced7cb861fa88a7c42bd)) - [@oknozor](https://github.com/oknozor)
- use structop instead of clap - ([b33598e](https://github.com/onagre-launcher/onagre/commit/b33598ea4319ca6d11714a028bba8a06b0e8cfed)) - [@oknozor](https://github.com/oknozor)
- remove unused menu theme and clap args - ([35add5f](https://github.com/onagre-launcher/onagre/commit/35add5f48ac0926b5109b5566d812ea35e00c77a)) - [@oknozor](https://github.com/oknozor)
- remove duplicate functions to get entry as row - ([3fd035a](https://github.com/onagre-launcher/onagre/commit/3fd035a258d7825cef7abe25badc46a497a74779)) - [@oknozor](https://github.com/oknozor)
- simplify icon path resolving - ([bfdbc25](https://github.com/onagre-launcher/onagre/commit/bfdbc25a2c2a0d922bd9e12f17d3b02446430403)) - [@oknozor](https://github.com/oknozor)
- simplify icon rendering - ([3d7e117](https://github.com/onagre-launcher/onagre/commit/3d7e117b7b5dbc0136cabd671dabdd60e6ae10dc)) - [@oknozor](https://github.com/oknozor)
- select row kind on Entry default impl - ([b0f18da](https://github.com/onagre-launcher/onagre/commit/b0f18daf670f7cb1794fe44f11851fa91075a963)) - [@oknozor](https://github.com/oknozor)
- remove border radius and add default bg constant - ([04b7836](https://github.com/onagre-launcher/onagre/commit/04b783670711fd3ca672a5a127ded3a452fb916c)) - [@oknozor](https://github.com/oknozor)
- remove modes - ([4d7906c](https://github.com/onagre-launcher/onagre/commit/4d7906c83686adcbe23060c4d6207e56c8ece036)) - [@oknozor](https://github.com/oknozor)
- use pop launcher for DE search - ([f295c2d](https://github.com/onagre-launcher/onagre/commit/f295c2d232c0b268baf180dcfa26993107e6405f)) - [@oknozor](https://github.com/oknozor)
- add cli app skeletton and fix entry alignment - ([78b821e](https://github.com/onagre-launcher/onagre/commit/78b821e935ed70547523e08150bf044bd2907491)) - [@oknozor](https://github.com/oknozor)
- extract app to a subomdule, preparing for cli - ([04d983b](https://github.com/onagre-launcher/onagre/commit/04d983b6e220a276a070cab34635fa33a64a2938)) - [@oknozor](https://github.com/oknozor)
- use std dedup to filter out duplicate entries - ([341324e](https://github.com/onagre-launcher/onagre/commit/341324e1311bb7bbd92db93f8eb9a592dcbe5fc5)) - [@oknozor](https://github.com/oknozor)
- use hashset for mode selection instead of index - ([18e15da](https://github.com/onagre-launcher/onagre/commit/18e15dac14e569da89f9ada4f30e0059a6b1ebc2)) - [@oknozor](https://github.com/oknozor)
- resolve entry by index instead of smartpointers - ([9bbb997](https://github.com/onagre-launcher/onagre/commit/9bbb9970f4da6760cde2cd125c3996749f41b307)) - [@oknozor](https://github.com/oknozor)
- use iced futures reexport instead of futures dependency - ([36bf43d](https://github.com/onagre-launcher/onagre/commit/36bf43d50eec4a3a3b04e0cdea0f24855b4cd02c)) - [@oknozor](https://github.com/oknozor)
- remove distinction between Drun and custom mode in state representation - ([40768a6](https://github.com/onagre-launcher/onagre/commit/40768a6774391a363db21a1b17036277568a1f0d)) - [@oknozor](https://github.com/oknozor)
- use Weak cell for entry matches - ([c9b6ef3](https://github.com/onagre-launcher/onagre/commit/c9b6ef3662c78491557e624482989c4003f60ae3)) - [@oknozor](https://github.com/oknozor)
- use a generic entry type for all modes - ([a7eb68e](https://github.com/onagre-launcher/onagre/commit/a7eb68eec954244fd28786eceff5fe287323d743)) - [@oknozor](https://github.com/oknozor)
- use option instead of result to get icon path - ([1da4839](https://github.com/onagre-launcher/onagre/commit/1da483992cc0ecf30ed2ceba5cf37799fae12653)) - [@oknozor](https://github.com/oknozor)
- remove some cloning and take ownedship when ref is not needed - ([4155ae9](https://github.com/onagre-launcher/onagre/commit/4155ae921f7b110d302455c4e81c4b2383cecb4e)) - [@oknozor](https://github.com/oknozor)
- use Rc to copy matches and remove rayon - ([cd0bdd2](https://github.com/onagre-launcher/onagre/commit/cd0bdd2bc79d975d2008fc648ce5bb55897aeb73)) - [@oknozor](https://github.com/oknozor)
- move entries to state - ([8a5b833](https://github.com/onagre-launcher/onagre/commit/8a5b8339b22a6f0e6760e9f2035057fb72a3f367)) - [@oknozor](https://github.com/oknozor)
- add generic ToRow trait - ([516d787](https://github.com/onagre-launcher/onagre/commit/516d7874be59f65419d0e065da18defc019a218c)) - [@oknozor](https://github.com/oknozor)
- use glob pattern to find desktop entries instead of async-std - ([ab36b64](https://github.com/onagre-launcher/onagre/commit/ab36b6413d710b64691159be29738340e2440299)) - [@oknozor](https://github.com/oknozor)
- work in progress to refactor icon finder - ([63a6a1c](https://github.com/onagre-launcher/onagre/commit/63a6a1cbcb6f7981d5fd66f4c91a5f32b47f3b89)) - [@oknozor](https://github.com/oknozor)
- reorganize stylesheets - ([3b1f5c1](https://github.com/onagre-launcher/onagre/commit/3b1f5c1e3b34e75287b3526aa97206dbb6a27bae)) - [@oknozor](https://github.com/oknozor)
- format and unused import - ([7a874e1](https://github.com/onagre-launcher/onagre/commit/7a874e1c16c0490888014dfed222cd1814f31079)) - [@oknozor](https://github.com/oknozor)
- extract ui building do dedicated function - ([a4e5973](https://github.com/onagre-launcher/onagre/commit/a4e59738e92410058273fdda8caf312e20fbcb4d)) - [@oknozor](https://github.com/oknozor)
- extract fs subsctiption do a dedicated module - ([35a7ae5](https://github.com/onagre-launcher/onagre/commit/35a7ae5c162cdc879d47f50ca6815c3c3d2ffaa3)) - [@oknozor](https://github.com/oknozor)
#### Tests
- cleanup unused test - ([83f81d7](https://github.com/onagre-launcher/onagre/commit/83f81d7f63030a9efba805820db9010d59c2c268)) - [@oknozor](https://github.com/oknozor)
- move bin to be able to benchmark icon finder - ([74d113a](https://github.com/onagre-launcher/onagre/commit/74d113a7d478a288822f70c11bcb78804acbe47a)) - [@oknozor](https://github.com/oknozor)

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).