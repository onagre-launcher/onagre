import hljs from 'highlight.js/lib/core';
import bash from './bash_with_onagre.js';
import ron from './ron.js';
import scss from 'highlight.js/lib/languages/scss';

hljs.registerLanguage('bash', bash);
hljs.registerLanguage('scss', scss);
hljs.registerLanguage('ron', ron);

export const highlightjsPlugin = () => ({
    name: '@vuepress/plugin-highlightjs',
    async extendsMarkdown(md) {
        md.options.highlight = (code, lang) => {
            if (lang === "text") {
                return code.value;
            } else {
                return hljs.highlight(code, {language: lang, ignoreIllegals: true}).value
            }
        }
    },
})
