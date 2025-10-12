import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  turbopack: {
    root: __dirname,
    rules: {
      "**/*.txt": {
        loaders: ["raw-loader"],
        as: '*.js',
      },
    },
  },
};

export default nextConfig;
