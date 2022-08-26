<template>
  <div
    @click="handleChange"
    :class="{
      'toggle-switch': true,
      '--checked': isChecked,
      '--unchecked': !isChecked,
    }"
  >
    <div
      :class="{
        'toggle-switch__slider': true,
        '--checked': isChecked,
        '--unchecked': !isChecked,
      }"
    ></div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    isChecked: {
      type: Boolean,
      default: false,
    },
    isDisable: {
      type: Boolean,
      default: false,
    },
  },
  emits: ['click-toggle-switch'],
  setup: (_, { emit }) => {
    const handleChange = (e: MouseEvent) => {
      emit('click-toggle-switch', e)
    }

    return { handleChange }
  },
})
</script>
<style scoped lang="scss">
.toggle-switch {
  @include clickable;
  position: relative;
  display: flex;
  align-items: center;
  width: 4.8rem;
  height: 2.8rem;
  border: solid 0.1rem $c-gray;
  border-radius: 100vw;
  &.--unchecked {
    background: $c-white;
  }
  &.--checked {
    background: $c-main-light;
  }
  &__slider {
    position: absolute;
    left: 0.3rem;
    width: 2.2rem;
    height: 2.2rem;
    background: $c-main;
    border-radius: 100vw;
    transition: all 300ms ease;
    &.--checked {
      transform: translate(1.8rem, 0);
    }
  }
}
</style>
