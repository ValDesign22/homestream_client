import globals from "globals";
import pluginJs from "@eslint/js";
import vueI18n from "@intlify/eslint-plugin-vue-i18n";
import tseslint from "typescript-eslint";
import pluginVue from "eslint-plugin-vue";

export default [
  {
    files: ["**/*.{js,mjs,cjs,ts,vue}"]
  },
  {
    languageOptions: {
      globals: globals.browser
    }
  },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  ...pluginVue.configs["flat/essential"],
  {
    files: ["**/*.vue"],
    languageOptions: {
      parserOptions: {
        parser: tseslint.parser
      }
    }
  },
  ...vueI18n.configs["flat/base"],
  {
    rules: {
      '@intlify/vue-i18n/no-dynamic-keys': 'error',
      '@intlify/vue-i18n/no-unused-keys': [
        'error',
        {
          extensions: ['.ts', '.js', '.vue'],
        },
      ],
      '@intlify/vue-i18n/no-raw-text': [
        'error',
        {
          'attributes': {
            '/.+/': [
              'title',
              'aria-label',
              'aria-placeholder',
              'aria-roledescription',
              'aria-valuetext',
            ],
            'input': ['placeholder'],
            'img': ['alt'],
          },
          'ignoreNodes': ['md-icon', 'v-icon'],
          'ignorePattern': '^[-#:()&]+$',
          'ignoreText': ['EUR', 'HKD', 'USD'],
        },
      ],
    },
    settings: {
      'vue-i18n': {
        localeDir: './src/translations/*.ts',
        messageSyntaxVersion: '^9.0.0',
      },
    },
  },
];