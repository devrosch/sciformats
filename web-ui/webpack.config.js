const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  mode: 'development',
  entry: {
    index: './src/index.ts',
  },
  devtool: 'source-map',
  devServer: {
    static: './dist',
    watchFiles: ['src/**/*.html', 'dist/**/*'],
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: 'Development',
      template: 'src/index.html',
    }),
  ],
  output: {
    filename: '[name].[chunkhash].bundle.js',
    path: path.resolve(__dirname, 'dist'),
    clean: true,
  },
  module: {
    rules: [
      // {
      //   test: /\.libsf.js$/i,
      //   type: 'asset/resource',
      // },
      // {
      //   test: /\.libsf.wasm$/i,
      //   type: 'asset/resource',
      // },
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader'],
      },
      {
        test: /\.(png|svg|jpg|jpeg|gif)$/i,
        type: 'asset/resource',
      },
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
    // to avoid relative paths to parent or sibling directories
    // https://medium.com/@etherealm/getting-rid-of-relative-paths-in-imports-using-webpack-alias-78d4bf15bb42
    // https://stackoverflow.com/questions/40443806/webpack-resolve-alias-does-not-work-with-typescript
    // https://medium.com/@martin_hotell/type-safe-es2015-module-import-path-aliasing-with-webpack-typescript-and-jest-fe461347e010
    alias: {
      assets: path.resolve(__dirname, 'src/assets/'),
      components: path.resolve(__dirname, 'src/components/'),
      model: path.resolve(__dirname, 'src/model/'),
      util: path.resolve(__dirname, 'src/util/'),
      worker: path.resolve(__dirname, 'src/worker/'),
    }
  },
  // optimization: {
  //   runtimeChunk: 'single',
  // },
  // experiments: {
  //   asyncWebAssembly: true,
  // },
};