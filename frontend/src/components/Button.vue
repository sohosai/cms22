<template>
  <button @click="handleClick" :style="`{width: ${width}}`" :class="[color]">
    <div :class="{ '--loading': loading, loader: true }">
      <div class="loader-inner ball-pulse-sync">
        <div></div>
        <div></div>
        <div></div>
      </div>
    </div>
    <div :class="{ '--loading': loading, text: true }">{{ text }}</div>
  </button>
</template>

<script lang="ts">
import 'loaders.css'
import { defineComponent, PropType } from 'vue'
type ButtonColor = 'primary' | 'secondary'

export default defineComponent({
  props: {
    text: {
      type: String,
      required: true,
    },
    width: {
      type: String,
      default: '20rem',
    },
    color: {
      type: String as PropType<ButtonColor>,
      default: 'primary',
    },
    loading: {
      type: Boolean,
      default: false,
    },
  },
  emits: ['click', 'update:loading'],
  setup(_, context) {
    const handleClick = (e: Event) => {
      context.emit('click', e)
    }
    return {
      handleClick,
    }
  },
})
</script>

<style lang="scss" scoped>
button {
  position: relative;
  padding: 1rem 2rem;
  text-align: center;
  vertical-align: middle;
  border-radius: 0.8rem;
  &:hover {
    opacity: 0.7;
  }
}
.text {
  &.--loading {
    opacity: 0.5;
  }
}
.primary {
  color: $c-white;
  background-color: $c-main;
}
.secondary {
  background-color: $c-gray-light;
}
.loader {
  display: none;
  &.--loading {
    display: block;
  }
}
.loader-inner {
  position: absolute;
  transform: scale(0.5, 0.5);
}
</style>
