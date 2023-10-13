import { resolve } from "path";

export default {
  bundler: "webpack",
  input: resolve(__dirname, "index.js"),
  server: {
    port: 8080,
  },
  polyfills: {
    buffer: true,
    url: true,
    punycode: true,
  },
  experimental: {},
  customizeWebpackConfig: (config) => {
    config.module.rules[1] = {
      test: /\.wasm$/,
      type: "asset/inline",
    };

    return config;
  },
};
