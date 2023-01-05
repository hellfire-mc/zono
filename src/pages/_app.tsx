import { AppProps } from "next/app";
import dynamic from "next/dynamic";
import React from "react";

import { ReduxProvider } from "../redux";
import { ThemeProvider } from "../theme";

const App = ({ Component, pageProps }: AppProps) => (
	<React.StrictMode>
		<ReduxProvider>
			<ThemeProvider>
				<Component {...pageProps} />
			</ThemeProvider>
		</ReduxProvider>
	</React.StrictMode>
);

export default App;
