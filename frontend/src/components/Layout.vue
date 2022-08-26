<template>
  <div class="layout">
    <Sidebar>
      <router-view></router-view>
    </Sidebar>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import Sidebar from '@/components/Sidebar.vue'

export default defineComponent({
  components: {
    Sidebar,
  },
})
</script>

<style lang="scss" scoped>
.sidebar {
  position: sticky;
  top: 0;
}
</style>
