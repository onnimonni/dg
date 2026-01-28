/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/serve/templates.rs",
    "./sample-page.html",
  ],
  theme: {
    extend: {
      colors: {
        'piper-dark': '#101524',
        'piper-card': '#1a202c',
        'piper-accent': '#007c43',
        'piper-light': '#00a55a',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
    },
  },
  plugins: [],
}
