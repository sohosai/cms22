<template>
  <div class="contact">
    <Breadcrumbs :navigations="navigations" />
    <Title title="お問い合わせ" />
    現在準備中です。<br />
    <a mailto="project47th@sohosai.com">project47th@sohosai.com</a
    >までお問い合わせください。
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue-demi'
import { paths } from '@/const/config'
import Breadcrumbs from '@/components/Breadcrumbs.vue'
import Title from './Title.vue'

export default defineComponent({
  components: {
    Breadcrumbs,
    Title,
  },
  setup() {
    const navigations = ref([
      { label: paths.top.label(), path: paths.top.path() },
      { label: paths.contact.label(), path: paths.contact.path() },
    ])
    return {
      navigations,
    }
  },
})
</script>
