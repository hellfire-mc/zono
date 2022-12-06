import { useEffect } from "react";

import { appWindow } from "@tauri-apps/api/window";

import { useAppDispatch } from "../redux/hooks";
import { Theme, updateTargetSystemTheme } from "../redux/slices/theme";
import { getLogger } from "../util/logging";

const logger = getLogger("ThemeListener");

export const SystemThemeListener = () => {
	const dispatch = useAppDispatch();

	useEffect(() => {
		const updateFromWindow = async (theme?: "dark" | "light") => {
			// get window theme
			const newTheme =
				theme ??
				(await appWindow.theme().catch(() => {
					logger.warn(
						"Failed to query for system theme - defaulting to light theme"
					);
					return Theme.Light;
				}));
			// switch on theme and update target system theme
			switch (newTheme) {
				case "light": {
					dispatch(updateTargetSystemTheme(Theme.Light));
				}
				case "dark": {
					dispatch(updateTargetSystemTheme(Theme.Dark));
				}
			}
		};
		// run on mount, and run on window theme changed
		updateFromWindow();
		const handler = appWindow
			.onThemeChanged((ev) => updateFromWindow(ev.payload))
			.catch(() =>
				logger.warn("Failed to register system theme event listener")
			);

		return () => {
			logger.debug("Cleaning up system theme event listener...");
			(async () => {
				const unlisten = await handler;
				if (unlisten) {
					unlisten();
				}
			})();
		};
	}, [dispatch]);

	return <></>;
};
