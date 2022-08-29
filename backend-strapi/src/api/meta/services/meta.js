'use strict';

/**
 * meta service
 */

const { createCoreService } = require('@strapi/strapi').factories;

module.exports = createCoreService('api::meta.meta');
