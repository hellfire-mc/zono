import React from "react";
import { Provider } from "react-redux";

import { configureStore } from "@reduxjs/toolkit";

import { instance } from "./slices/instance";
import { theme } from "./slices/theme";

const store = configureStore({
	reducer: {
		theme: theme.reducer,
		instance: instance.reducer,
	},
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export const ReduxProvider: React.FC<{ children: React.ReactNode }> = ({
	children,
}) => <Provider store={store}>{children}</Provider>;
