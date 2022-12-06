import "./App.css";

import React from "react";
import { RouterProvider } from "react-router-dom";

import { ReduxProvider } from "./redux";
import { router } from "./router";
import { ThemeProvider } from "./theme";

export const App = () => (
	<React.StrictMode>
		<ReduxProvider>
			<ThemeProvider>
				<RouterProvider router={router} />
			</ThemeProvider>
		</ReduxProvider>
	</React.StrictMode>
);
