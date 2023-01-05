/** @type {import('next').NextConfig} */

const nextConfig = {
	reactStrictMode: true,
	swcMinify: true,
	// Note: This experimental feature is required to use NextJS Image in SSG mode.
	// See https://nextjs.org/docs/messages/export-image-api for different workarounds.
	images: {
		unoptimized: true,
	},
	async redirects() {
		return [
			{
				source: "/",
				destination: "/instances",
				permanent: true,
			},
		];
	},
};

module.exports = nextConfig;
