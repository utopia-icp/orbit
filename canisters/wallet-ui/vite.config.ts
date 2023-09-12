import inject from '@rollup/plugin-inject';
import vue from '@vitejs/plugin-vue';
import { readdirSync } from 'fs';
import { basename, dirname, resolve } from 'path';
import vuetify from 'vite-plugin-vuetify';
import { defineConfig } from 'vitest/config';

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  const isProduction = mode === 'production';
  const localesPath = resolve(__dirname, 'src/locales');
  const supportedLocales = readdirSync(localesPath).map(file => basename(file, '.json'));

  return {
    base: '/',
    root: '.',
    publicDir: './public',
    appType: 'spa',
    // Vite automatically loads .env files from the root of the project
    // if they are prefixed with the envPrefix.
    envPrefix: 'APP_',
    plugins: [vue(), vuetify({ autoImport: true })],
    build: {
      target: 'es2020',
      sourcemap: !isProduction,
      minify: isProduction,
      chunkSizeWarningLimit: 500,
      outDir: './dist',
      emptyOutDir: true,
      rollupOptions: {
        input: {
          main: './index.html',
        },
        output: {
          manualChunks: id => {
            const folder = dirname(id);
            const isNodeModule = folder.includes('node_modules');

            if (
              folder.includes('/src/locales') &&
              supportedLocales.some(locale => resolve(folder, `${locale}.json`) === id)
            ) {
              const [localeName] = basename(id).split('.');
              return `locale-${localeName}`;
            }

            if (
              isNodeModule &&
              [
                '/@vue/',
                '/vue/',
                '/vue-router/',
                '/vue-demi/',
                '/pinia/',
                '/vuetify/',
                '/vue-i18n/',
                '/@intlify/',
              ].some(vendor => folder.includes(vendor))
            ) {
              return 'vendor';
            }
          },
        },
        plugins: [
          inject({
            modules: {
              // Polyfill Buffer for production build
              Buffer: ['buffer', 'Buffer'],
            },
          }),
        ],
      },
    },
    css: {
      devSourcemap: !isProduction,
    },
    test: {
      globals: true,
      environment: 'happy-dom',
      setupFiles: ['./setupFiles/GlobalsConfiguration.ts'],
    },
    define: {
      // Vite env variable replacements for the runtime.
      //
      // Make sure to use import.meta.env as the prefix since
      // vite uses that during runtime to access the variables.
      // https://vitejs.dev/guide/env-and-mode.html#env-variables
      'import.meta.env.APP_VERSION': JSON.stringify(`v${process.env.npm_package_version}`),
    },
    resolve: {
      alias: {
        '~': resolve('src'),
      },
    },
  };
});
