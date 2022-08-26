<template>
  <div class="hint-tip">
    <span :class="['icon', 'material-icons', color]">help_outline</span>
    <div class="hint">
      <slot />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    color: {
      type: String,
      default: 'gray',
    },
  },
})
</script>

<style lang="scss" scoped>
.hint-tip {
  display: inline-block;
  vertical-align: middle;
  &:hover {
    .hint {
      z-index: 1000;
      transition: all 0.3s ease;
      opacity: 1;
    }
  }
}
.icon {
  color: $c-text-sub;
  cursor: default;
  &.gray {
    color: $c-text;
  }
  &.white {
    color: $c-gray;
  }
}
.hint {
  @include fs-0;
  position: absolute;
  z-index: -1;
  top: 2rem;
  right: 2rem;
  opacity: 0;
  padding: 2rem;
  border-radius: 0.4rem;
  box-shadow: -0.1rem -0.1rem 0.2rem $c-shadow, 0.3rem 0.3rem 0.6rem $c-shadow;
  width: 40rem;
  background-color: $c-white;
  word-break: break-all;
  color: $c-text;
  line-height: 1.7;
  transition: all 0.3s ease;
  ::v-slotted(.heading) {
    @include fs-0b;
    padding-top: 0.5rem;
  }
}
</style>
