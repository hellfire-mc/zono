import { VStack } from "@chakra-ui/react";

import { InstanceGroup } from "../../components/instances/InstanceGroup";
import { PageLayout } from "../../layout/PageLayout";
import { useInstanceGroups } from "../../redux/slices/instance";

const Instances = () => {
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

export default Instances;
