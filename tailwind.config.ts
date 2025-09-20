import type { Config } from "tailwindcss";

const config: Config = {
	theme: {
		fontFamily: {
			sans: ["Inter", "ui-sans-serif", "system-ui"],
		},
		extend: {
			colors: {
				primary: {
					50: "#fef7f6",
					100: "#fdeeed",
					200: "#fbd5d1",
					300: "#f8bcb5",
					400: "#fc5647",
					500: "#e74c3c",
					600: "#c0392b",
					700: "#a93226",
					800: "#922b20",
					900: "#7b241a",
				},

				// Neutrals
				neutral: {
					50: "#f4f3f0",
					100: "#e8e6e1",
					200: "#d1cdc3",
					300: "#bab4a5",
					400: "#a39b87",
					500: "#8c8269",
					600: "#666666",
					700: "#4a4a4a",
					800: "#2d2d2d",
					900: "#000000",
				},

				// Accent colors
				accent: {
					blue: "#1c7ff2",
					green: "#1c9b48",
					lime: "#e1f37c",
					purple: "#dab8f4",
				},

				// Semantic colors
				background: {
					primary: "#ffffff",
					secondary: "#f4f3f0",
					tertiary: "#e8e6e1",
				},

				text: {
					primary: "#000000",
					secondary: "#666666",
					tertiary: "#8c8269",
					inverse: "#ffffff",
				},
			}
		},
	},
	plugins: [],
};

export default config;
