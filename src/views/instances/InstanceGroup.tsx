import { Box, Heading, HStack } from "@chakra-ui/react";

import { InstanceCard } from "./InstanceCard";

export const InstanceGroup: React.FC<{ name: string; instances: string[] }> = ({
	name,
	instances,
}) => {
	const cards = instances.map((id) => <InstanceCard key={id} id={id} />);

	return (
		<Box width="100%">
			<Heading size="md" marginBottom={5}>
				{name}
			</Heading>
			<HStack>{cards}</HStack>
		</Box>
	);
};
