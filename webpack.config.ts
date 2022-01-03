import * as webpack from "webpack";
import * as path from "path";
import HtmlWebpackPlugin from "html-webpack-plugin";

let mode: 'production' | 'development';
if (process.env.MODE_ENV === 'prod') {
  mode = 'production';
} else {
  mode = 'development';
}
const config: webpack.Configuration = {
  target: "web",
  entry: "./index.ts",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js",
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: "ts-loader",
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({template: "./public/index.html"}),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
  resolve: {
    extensions: ['.ts', '.js'],
  },
  mode,
  devtool: false,
}

export default config;
