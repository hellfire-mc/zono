"use client";

import React from "react";

import { createClient, NoOpTransport } from "@rspc/client";
import { createReactQueryHooks } from "@rspc/react";
import { QueryClient } from "@tanstack/react-query";

import { ClientTauriTransport } from "./transport";

import type { Procedures } from "./types.generated";
const client = createClient<Procedures>({
	transport:
		// Condition on window - nextjs renders on server and hydrates on client
		// so queries will attempted to be run during build-time.
		typeof window === "undefined"
			? new NoOpTransport()
			: new ClientTauriTransport(),
});
const queryClient = new QueryClient();

export const RspcProvider: React.FC<{ children: React.JSX.Element }> = ({
	children,
}) => (
	<Provider client={client} queryClient={queryClient}>
		{children}
	</Provider>
);

export const { useContext, useMutation, useQuery, useSubscription, Provider } =
	createReactQueryHooks<Procedures>();
