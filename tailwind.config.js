/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
    './node_modules/preline/dist/*.js',
  ],
  darkMode: 'media',
  // darkMode: 'media',

  theme: {
    extend: {
      // extend base Tailwind CSS utility classes
    },
  },

  // add plugins to your Tailwind CSS project
  plugins: [
    require('@tailwindcss/forms'),
    require('preline/plugin')
  ],
}

