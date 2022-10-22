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
    // https://webpack.js.org/configuration/module/#condition
    // https://webpack.js.org/configuration/resolve/#resolveroots
    // but:
    // https://stackoverflow.com/questions/40443806/webpack-resolve-alias-does-not-work-with-typescript
    alias: {
      model: path.resolve(__dirname, 'src/model/'),
      components: path.resolve(__dirname, 'src/components/'),
    }
  },
  // optimization: {
  //   runtimeChunk: 'single',
  // },
};