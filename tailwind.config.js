/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],
  theme: {
    extend: {
      keyframes: {
        highlight: {
          "0%": { background: "#8f8" },
          "100%": { background: "auto" },
        },
      },
      animation: { highlight: "highlight 1s" },
    },
  },
  plugins: [],
};
