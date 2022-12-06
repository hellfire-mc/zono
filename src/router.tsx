import { createBrowserRouter } from "react-router-dom";

import Dashboard from "./views/Dashboard";

export const router = createBrowserRouter([
	{
		path: "/",
		element: <Dashboard />,
	},
]);
