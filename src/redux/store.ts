import { configureStore } from "@reduxjs/toolkit";

import { instance } from "./slices/instance";
import { theme } from "./slices/theme";

export const store = configureStore({
	reducer: {
		theme: theme.reducer,
		instance: instance.reducer,
	},
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
