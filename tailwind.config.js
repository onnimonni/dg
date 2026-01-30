/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/serve/templates.rs",
    "./src/serve/generator.rs",
    "./sample-page.html",
  ],
  // Safelist dynamically generated classes from generator.rs
  safelist: [
    'text-emerald-500',
    'text-red-500',
    'text-amber-500',
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
