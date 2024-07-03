/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "src/pages/*.rs"],
  },
  theme: {
    extend: {
      colors: {  // переопределяем цвета
        neutral: {  // группы neutral
          100: '#F1F1F1',
          800: '#202020'
        }
      }
    },
  },
  plugins: [],
}