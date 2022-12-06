import { extendTheme } from "@chakra-ui/react";

import { BASE_THEME } from "./base";

const config = {
	initialColorMode: "dark",
};

export const LIGHT_THEME = extendTheme({ config }, BASE_THEME);
