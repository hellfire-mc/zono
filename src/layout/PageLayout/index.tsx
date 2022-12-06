import { Box, HStack, VStack } from "@chakra-ui/react";

import { AppBar } from "./AppBar";
import { Sidebar } from "./Sidebar";

export const PageLayout: React.FC<{
	title: string;
	children?: React.ReactNode;
}> = ({ title, children }) => {
	return (
		<HStack height="100vh" width="100vw" spacing={0}>
			<Sidebar />
			<VStack flexGrow={1} height="100%">
				<AppBar title={title} />
				<Box width="100%">{children}</Box>
			</VStack>
		</HStack>
	);
};
