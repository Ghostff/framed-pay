/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
    './node_modules/preline/dist/*.js',
  ],
  darkMode: 'class',
  // darkMode: 'media',

  theme: {
    extend: {
      // extend base Tailwind CSS utility classes
      fontFamily: {
        sans: [
          'Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Helvetica Neue, Arial, Noto Sans, sans-serif, Apple Color Emoji, Segoe UI Emoji, Segoe UI Symbol, Noto Color Emoji'
        ]
      }
    },
  },

  // add plugins to your Tailwind CSS project
  plugins: [
    require('@tailwindcss/forms'),
    require('preline/plugin')
  ],
}

