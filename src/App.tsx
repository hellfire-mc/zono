import "./App.css";

import React from "react";
import { RouterProvider } from "react-router-dom";

import { CssBaseline } from "@mui/material";

import { ReduxProvider } from "./redux";
import { router } from "./router";
import { ThemeProvider } from "./theme";

export const App = () => (
	<React.StrictMode>
		<ReduxProvider>
			<CssBaseline />
			<ThemeProvider>
				<RouterProvider router={router} />
			</ThemeProvider>
		</ReduxProvider>
	</React.StrictMode>
);
