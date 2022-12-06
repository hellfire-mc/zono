import { VStack } from "@chakra-ui/react";

import { PageLayout } from "../../layout/PageLayout";
import { useInstanceGroups } from "../../redux/slices/instance";
import { InstanceGroup } from "./InstanceGroup";

export const Instances = () => {
	const groups = useInstanceGroups().map(({ name, instances }, id) => (
		<InstanceGroup key={id} name={name} instances={instances} />
	));

	return (
		<PageLayout title="Instances">
			<VStack padding={4} spacing={4} overflowY="scroll">
				{groups}
			</VStack>
		</PageLayout>
	);
};
