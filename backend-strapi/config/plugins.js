
module.exports = {
  "rest-cache": {
    config: {
      provider: {
        name: "memory",
        options: {
          max: 32767,
          maxAge: 600,
        },
      },
      strategy: {
        contentTypes: [
          "api::content.content",
        ],
      },
    },
  },
};
