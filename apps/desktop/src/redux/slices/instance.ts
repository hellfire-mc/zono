import { createSlice } from "@reduxjs/toolkit";

import { useAppSelector } from "../hooks";

interface Instance {
	id: string;
	name: string;
}

interface InstanceGroup {
	name: string;
	instances: string[];
}

interface InstanceState {
	instances: Record<string, Instance>;
	groups: InstanceGroup[];
}

export const instance = createSlice({
	name: "instance",
	initialState: {
		instances: {
			"1234": { id: "1234", name: "1.19.2" },
		},
		groups: [{ name: "Favourites", instances: ["1234"] }],
	} as InstanceState,
	reducers: {},
});

export const useInstance = (id: string) =>
	useAppSelector((state) => state.instance.instances[id]);

export const useInstanceGroups = () =>
	useAppSelector((state) => state.instance.groups);
