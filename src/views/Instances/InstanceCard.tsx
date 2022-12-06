import { Heading } from "@chakra-ui/react";
import { Card, CardHeader } from "@saas-ui/react";

import { useInstance } from "../../redux/slices/instance";

export const InstanceCard: React.FC<{ id: string }> = ({ id }) => {
	const instance = useInstance(id);

	return (
		<Card minWidth="300px" minHeight="200px">
			<CardHeader>
				<Heading size="sm">{instance.name}</Heading>
			</CardHeader>
		</Card>
	);
};
