<template>
  <div class="text-input">
    <div v-if="label" class="label">{{ label }}</div>
    <input type="text" @input="handleChange" :value="modelValue" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    modelValue: {
      type: String,
      required: true,
    },
    label: {
      type: String,
      default: '',
    },
  },
  emits: ['update:modelValue'],
  setup(_, context) {
    const handleChange = (e: Event) => {
      if (!(e.target instanceof HTMLInputElement)) {
        return
      }
      context.emit('update:modelValue', e.target.value)
    }
    return {
      handleChange,
    }
  },
})
</script>

<style lang="scss" scoped>
.text-input {
  width: 100%;
}
input {
  width: 100%;
  padding: 1rem;
  border: 1px solid $c-gray-dark;
  border-radius: 0.4rem;
  &:focus {
    border-color: $c-main;
  }
}
.label {
  margin-bottom: 1rem;
}
</style>
