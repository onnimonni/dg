/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/serve/templates.rs",
    "./sample-page.html",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
    },
  },
  plugins: [],
}
