import Link from "next/link";
import { AiFillExperiment, AiTwotoneAppstore, AiTwotoneSetting } from "react-icons/ai";

import { Divider, HStack, Icon, List, ListItem, Text, VStack } from "@chakra-ui/react";

const NAV_LINKS = [
	{
		icon: AiTwotoneAppstore,
		text: "Instances",
		to: "/",
	},
	{
		icon: AiFillExperiment,
		text: "Mods",
		to: "/mods",
	},
	{
		icon: AiTwotoneSetting,
		text: "Settings",
		to: "/settings",
	},
];

export const Sidebar: React.FC<{}> = () => {
	return (
		<VStack
			padding={4}
			height="100%"
			minWidth="250px"
			borderRight="1px solid"
			borderColor="#eee"
		>
			<List spacing={2} width="100%">
				{NAV_LINKS.map(({ icon, text, to }, i) => (
					<ListItem key={i}>
						<Link href={to}>
							<HStack>
								<Icon as={icon} />
								<Text size="sm">{text}</Text>
							</HStack>
						</Link>
					</ListItem>
				))}
				<Divider />
			</List>
		</VStack>
	);
};
