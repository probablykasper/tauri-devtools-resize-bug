import preprocess from 'svelte-preprocess'
import adapter from '@sveltejs/adapter-static'

/** @type {import('@sveltejs/kit').Config} */
export default {
  preprocess: preprocess(),

  kit: {
    target: '#svelte',
    ssr: false,
    prerender: {
      enabled: false,
    },
    adapter: adapter({
      pages: '.svelte-kit/build-output',
      // fallback: 'app.html',
    }),
    vite: {
      clearScreen: false,
    },
  },
}
