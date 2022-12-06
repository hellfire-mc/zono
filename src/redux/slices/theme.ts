import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export enum Theme {
	Light,
	Dark,
}

interface ThemeState {
	active: Theme;
	targetSystem: Theme;
	useSystem: boolean;
}

export const theme = createSlice({
	name: "theme",
	initialState: {
		active: Theme.Light,
		useSystem: false,
	} as ThemeState,
	reducers: {
		updateTargetSystemTheme: (state, action: PayloadAction<Theme>) => {
			state.targetSystem = action.payload;
		},
		shouldUseSystemTheme: (state, action: PayloadAction<boolean>) => {
			state.useSystem = action.payload;
			// update state if necessary
			if (state.useSystem) {
				state.active = state.targetSystem;
			}
		},
		setTheme: (state, action: PayloadAction<Theme>) => {
			if (state.useSystem) {
				return;
			}
			state.active = action.payload;
		},
	},
});

export const { updateTargetSystemTheme, shouldUseSystemTheme, setTheme } =
	theme.actions;
