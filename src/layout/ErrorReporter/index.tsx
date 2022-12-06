import React, { useEffect } from "react";
import { useRouteError } from "react-router-dom";

import { Button, ButtonGroup, Center, Code, Heading, Text, VStack } from "@chakra-ui/react";

import { getLogger } from "../../util/logging";

const logger = getLogger("ErrorReporter");

export const ErrorReporter = () => {
	const error = useRouteError();

	useEffect(() => {
		logger.info("Caught error at error boundary - uploading report...", error);
	}, []);

	return (
		<Center width="100vw" height="100vh">
			<VStack maxWidth="600px" spacing={8}>
				<Heading size="lg">Whoops...</Heading>
				<Text textAlign="center">
					Looks like you've encountered an error. We&apos;ve gone ahead and
					reported this for you, so it should be fixed soon!
				</Text>
				<Text>
					Reference: <Code>xgdSHb62a</Code>
				</Text>
				<ButtonGroup>
					<Button size="lg" variant="solid" colorScheme="purple">
						Reload
					</Button>
					<Button size="lg" variant="outline">
						Exit
					</Button>
				</ButtonGroup>
			</VStack>
		</Center>
	);
};
