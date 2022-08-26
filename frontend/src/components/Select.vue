<template>
  <div class="select">
    <select :value="modelValue" @change="handleChange">
      <option disabled value>選択してください</option>
      <option
        v-for="option in options"
        :key="option.value"
        :value="option.value"
      >
        {{ option.label }}
      </option>
    </select>
    <div class="material-icons">expand_more</div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue'

type Options = {
  label: string
  value: string
}[]

export default defineComponent({
  props: {
    modelValue: {
      type: String,
      required: true,
    },
    options: {
      type: Object as PropType<Options>,
      required: true,
    },
  },
  emit: ['update:modelValue'],
  setup(_, context) {
    const handleChange = (e: Event) => {
      if (!(e.target instanceof HTMLSelectElement)) {
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
.select {
  @include center;
  justify-content: space-between;
  border: 1px solid $c-gray-dark;
  border-radius: 0.4rem;
}
select {
  padding: 1rem;
  width: 100%;
}
</style>
