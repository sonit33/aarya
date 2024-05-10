/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["./pages/**/*.hbs"],
	theme: {
		extend: {
			fontFamily: {
				sans: ["Alliance", "sans-serif"],
				mono: ['"Fira Code"', "monospace"],
			},
		},
	},
	plugins: [require("@tailwindcss/typography")],
};
