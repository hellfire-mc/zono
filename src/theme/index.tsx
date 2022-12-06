import React from "react";

import { ThemeProvider as EmotionThemeProvider } from "@emotion/react";

import { useAppSelector } from "../redux/hooks";
import { Theme } from "../redux/slices/theme";
import { DARK_THEME } from "./dark";
import { LIGHT_THEME } from "./light";
import { SystemThemeListener } from "./system";

export const useTheme = () => {
	const theme = useAppSelector((state) => state.theme.active);
	switch (theme) {
		case Theme.Dark:
			return DARK_THEME;
		case Theme.Light:
			return LIGHT_THEME;
	}
};

export const ThemeProvider: React.FC<{
	children: React.ReactNode;
}> = ({ children }) => {
	const theme = useTheme();

	return (
		<>
			<SystemThemeListener />
			<EmotionThemeProvider theme={theme}>{children}</EmotionThemeProvider>
		</>
	);
};
