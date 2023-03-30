export const getLogger = (name: string) => ({
	info: console.info.bind(console, `%c[${name}]`, "color: magenta;"),
	warn: console.warn.bind(console, `%c[${name}]`, "color: magenta;"),
	error: console.error.bind(console, `%c[${name}]`, "color: magenta;"),
	debug: console.debug.bind(console, `%c[${name}]`, "color: magenta;"),
});
