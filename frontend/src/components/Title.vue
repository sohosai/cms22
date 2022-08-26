<template>
  <div class="title">{{ title }}</div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    title: {
      type: String,
      required: true,
    },
  },
})
</script>

<style lang="scss" scoped>
.title {
  @include fs-3;
  color: $c-text-sub;
}
</style>
