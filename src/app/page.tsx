"use client";

import { useQuery } from "../rspc";

export default function HomePage() {
	const { data, error, isLoading } = useQuery(["version"]);

	if (isLoading) {
		return <div>Loading...</div>;
	}

	if (error) {
		return <div>Error: {error.message}</div>;
	}

	return (
		<div>
			<h1>{data}</h1>
		</div>
	);
}
