/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  theme: {
		fontFamily: {
			'sans': ['"Noto Sans SC"', 'sans-serif'],
			'serif': ['"Noto Serif SC"', 'serif'],
			'cursive': ['"Liu Jian Mao Cao"', 'cursive', 'sans-serif'],
		},
    extend: {},
  },
  plugins: [],
}
