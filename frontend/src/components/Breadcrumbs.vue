<template>
  <nav class="breadcrumbs">
    <div v-for="(nav, i) in navigations" :key="nav.path" class="crumb">
      <a :href="nav.path">{{ nav.label }}</a>
      <span v-if="i < navigations.length - 1">&gt;</span>
    </div>
  </nav>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    navigations: {
      type: Array,
      required: true,
    },
  },
})
</script>

<style lang="scss" scoped>
.breadcrumbs {
  @include center;
  justify-content: flex-start;
  width: 100%;
  padding: 2rem 0;
  color: $c-text-sub;
}
a {
  &:hover {
    text-decoration: underline;
  }
}
span {
  padding: 0 1rem;
}
</style>
