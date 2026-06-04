import daisyui from 'daisyui';

export default {
  content: [
    './index.html',
    './src/**/*.svelte',
    './src/**/*.ts',
  ],
  theme: {
    extend: {},
  },
  plugins: [daisyui],
};
