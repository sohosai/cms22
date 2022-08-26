<template>
  <router-view v-slot="{ Component }">
    <template v-if="Component">
      <suspense>
        <component :is="Component"></component>
        <template #fallback>
          <Loading />
        </template>
      </suspense>
    </template>
  </router-view>
</template>

<script lang="ts">
import { defineComponent } from 'vue-demi'
import Loading from './components/ScreenLoading.vue'

export default defineComponent({
  components: {
    Loading,
  },
})
</script>

<style lang="scss" scoped></style>
