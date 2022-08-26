<template>
  <div class="reflect-authority">{{ message }}</div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import { reflectAuthority } from '@/utls/reflectAuthority'

export default defineComponent({
  async setup() {
    const res = await reflectAuthority()
    const message = ref<string>(res.ok ? '権限を更新しました。' : res.message)

    return {
      message,
    }
  },
})
</script>

<style lang="scss" scoped>
.reflect-authority {
  white-space: pre-line;
}
</style>
