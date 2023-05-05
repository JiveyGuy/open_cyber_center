/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/main.ts",
    "./src/**/*.{css,vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        "Background"   : "#282a36", 
        "CurrentLine"  : "#44475a",
        "Selection"    : "#44475a", 
        "Foreground"   : "#f8f8f2", 
        "Comment"      : "#6272a4", 
        "Cyan"         : "#8be9fd", 
        "Green"        : "#50fa7b", 
        "Orange"       : "#ffb86c", 
        "Pink"         : "#ff79c6", 
        "Purple"       : "#bd93f9", 
        "Red"          : "#ff5555", 
        "Yellow"       : "#f1fa8c"
      },
    }
  },
  dropShadow: {
    glow: [
      "0 0px 20px rgba(255,255, 255, 0.35)",
      "0 0px 65px rgba(255, 255,255, 0.2)",
    ],
  }
  
}