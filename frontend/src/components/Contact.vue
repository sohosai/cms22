<template>
  <div class="contact">
    <Breadcrumbs :navigations="navigations" />
    <Title title="お問い合わせ" />
    本システムは、筑波大学学園祭実行委員会
    情報メディアシステム局が管理しております。
    運用には万全を期しておりますが、お気づきの点やご質問のある方は<br />
    <a mailto="project48th@sohosai.com">project48th@sohosai.com</a
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
