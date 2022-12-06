import { createBrowserRouter } from "react-router-dom";

import { Dashboard } from "./views/Instances";

export const router = createBrowserRouter([
	{
		path: "/",
		element: <Dashboard />,
	},
]);
