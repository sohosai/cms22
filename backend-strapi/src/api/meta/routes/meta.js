'use strict';

/**
 * meta router
 */

const { createCoreRouter } = require('@strapi/strapi').factories;

module.exports = createCoreRouter('api::meta.meta');
