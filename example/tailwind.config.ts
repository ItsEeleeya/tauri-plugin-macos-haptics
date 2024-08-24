import type { Config } from "tailwindcss";
import daisyui from "daisyui";

const config = {
  darkMode: ["class"],
  content: [
    "./pages/**/*.{ts,tsx}",
    "./components/**/*.{ts,tsx}",
    "./app/**/*.{ts,tsx}",
    "./src/**/*.{ts,tsx}",
  ],
  prefix: "",
  plugins: [require("tailwindcss-animate"), daisyui],
} satisfies Config;

export default config;
