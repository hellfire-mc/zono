import { extendTheme } from "@chakra-ui/react";

import { BASE_THEME } from "./base";

const config = {
	initialColorMode: "dark",
};

export const DARK_THEME = extendTheme({ config }, BASE_THEME);
