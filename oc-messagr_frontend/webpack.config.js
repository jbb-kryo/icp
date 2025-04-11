const path = require("path");
const webpack = require("webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

const isDevelopment = process.env.NODE_ENV !== "production";

module.exports = {
    target: "web",
    mode: isDevelopment ? "development" : "production",
    entry: {
        main: "./src/index.jsx",
    },
    devtool: isDevelopment ? "source-map" : false,
    optimization: {
        minimize: !isDevelopment,
        minimizer: [new TerserPlugin()],
    },
    resolve: {
        extensions: [".js", ".jsx"],
        fallback: {
            assert: require.resolve("assert/"),
            buffer: require.resolve("buffer/"),
            events: require.resolve("events/"),
            stream: require.resolve("stream-browserify/"),
            util: require.resolve("util/"),
        },
    },
    output: {
        filename: "index.js",
        path: path.join(__dirname, "dist", "messagr_frontend"),
    },
    module: {
        rules: [
            {
                test: /\.(js|jsx)$/,
                exclude: /node_modules/,
                use: {
                    loader: "babel-loader",
                    options: {
                        presets: ["@babel/preset-env", "@babel/preset-react"],
                    },
                },
            },
            {
                test: /\.css$/,
                use: ["style-loader", "css-loader", "postcss-loader"],
            },
            {
                test: /\.svg$/,
                use: ["@svgr/webpack"],
            },
            {
                test: /\.(jpg|jpeg|png|gif|mp3|ico)$/,
                use: ["file-loader"],
            },
        ],
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "./src/index.html",
            filename: "index.html",
            chunks: ["main"],
        }),
        new webpack.ProvidePlugin({
            Buffer: [require.resolve("buffer/"), "Buffer"],
            process: require.resolve("process/browser"),
        }),
        new CopyPlugin({
            patterns: [
                {
                    from: path.join(__dirname, "src", "assets"),
                    to: path.join(__dirname, "dist", "messagr_frontend"),
                },
            ],
        }),
    ],
    devServer: {
        proxy: {
            "/api": "http://localhost:8000",
        },
        hot: true,
        watchFiles: ["src/**/*"],
        static: {
            directory: path.join(__dirname, "src", "assets"),
        },
        historyApiFallback: true,
    },
};