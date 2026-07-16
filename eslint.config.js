import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import svelte from "eslint-plugin-svelte";

export default tseslint.config(
    {
        ignores: [
            "build/**",
            "dist/**",
            ".svelte-kit/**",
            "src-tauri/target/**",
            "node_modules/**",
        ],
    },

    js.configs.recommended,
    ...tseslint.configs.recommended,
    ...svelte.configs["flat/recommended"],

    {
        files: ["**/*.svelte"],
        languageOptions: {
            parserOptions: {
                parser: tseslint.parser,
            },
        },
    },

    {
        languageOptions: {
            globals: {
                ...globals.browser,
            },
        },
    },

    {
        files: ["vite.config.js"],
        languageOptions: {
            globals: {
                ...globals.node,
            },
        },
    },
);
