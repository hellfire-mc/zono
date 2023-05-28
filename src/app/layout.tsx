"use client";

import React from "react";

import { ReduxProvider } from "../redux";
import { RspcProvider } from "../rspc";
import { ThemeProvider } from "../theme";

const Provider = ({ children }: { children: React.ReactNode }) => (
	<React.StrictMode>
		<ReduxProvider>
			<RspcProvider>
				<ThemeProvider>{children}</ThemeProvider>
			</RspcProvider>
		</ReduxProvider>
	</React.StrictMode>
);

export default function RootLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	return (
		<html lang="en">
			<body>
				<Provider>{children}</Provider>
			</body>
		</html>
	);
}
