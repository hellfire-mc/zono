import { Heading, HStack } from "@chakra-ui/react";

export const AppBar: React.FC<{ title: string }> = ({ title }) => (
	<HStack padding={4} width="100%">
		<Heading size="lg">{title}</Heading>
	</HStack>
);
