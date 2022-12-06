import { createSlice } from "@reduxjs/toolkit";

interface InstanceState {}

export const instance = createSlice({
	name: "instance",
	initialState: {} as InstanceState,
	reducers: {},
});
