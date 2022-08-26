<template>
  <label class="checkbox-label">
    <input type="checkbox" @change="handleChange" :checked="modelValue" />
    <span class="checkmark"></span>
    {{ label }}
  </label>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    modelValue: {
      type: Boolean,
      required: true,
    },
    label: {
      type: String,
      required: true,
    },
  },
  emits: ['update:modelValue'],
  setup(_, context) {
    const handleChange = (e: Event) => {
      if (!(e.target instanceof HTMLInputElement)) {
        return
      }
      context.emit('update:modelValue', e.target.checked)
    }
    return {
      handleChange,
    }
  },
})
</script>

<style lang="scss" scoped>
.checkbox-label {
  display: inline-block;
  position: relative;
  margin: 1rem 0;
  padding-left: 27px;
  cursor: pointer;

  .checkmark {
    position: absolute;
    left: 0;
    height: 1.4em;
    width: 1.4em;
    border: 2px solid $c-gray-dark;
    border-radius: 0.4rem;

    &::after {
      box-sizing: content-box;
      content: '';
      position: absolute;
      top: 0;
      left: 5px;
      width: 6px;
      height: 12px;
      border: solid #fff;
      border-width: 0 2px 2px 0;
      transform: rotate(45deg);
      opacity: 0;
    }
  }
  input[type='checkbox'] {
    display: none;

    &:checked + .checkmark {
      background: $c-main;
      border-color: $c-main;

      &:after {
        opacity: 1;
      }
    }
  }
}
</style>
