module.exports = {
  content: [
    './src/frontend/public/index.html',
    './src/frontend/src/**/*.svelte',
  ],
  theme: {
    extend: {},
  },
  plugins: [require('daisyui'), require('@tailwindcss/line-clamp')],
};
