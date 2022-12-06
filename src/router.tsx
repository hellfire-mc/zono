import { createBrowserRouter } from "react-router-dom";

import { ErrorReporter } from "./layout/ErrorReporter";
import { Instances } from "./views/instances";

export const router = createBrowserRouter([
	{
		path: "/",
		element: <Instances />,
		errorElement: <ErrorReporter />,
	},
	{
		path: "*",
		loader: () => {
			throw new Error("Unreachable");
		},
		errorElement: <ErrorReporter />,
	},
]);
