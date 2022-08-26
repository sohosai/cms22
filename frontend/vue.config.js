module.exports = {
  css: {
    loaderOptions: {
      sass: {
        prependData: `
          @import "@/assets/css/_utl.scss";
        `,
      },
    },
  },
  devServer: {
    port: 8081,
  },
}
