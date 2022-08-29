<template>
  <div class="faq">
    <Breadcrumbs :navigations="navigations" />
    <Title title="よくある質問" />
    <section class="body">
      <FaqItem>
        <template #question>ログインできません</template>
        <template #answer
          >本サービスは、 雙峰祭オンラインシステムと連携動作しています。
          アカウントやパスワードは、雙峰祭オンラインシステムと同様です。また、同システム上で「責任者」または「副責任者」となっている企画のみ編集できます。</template
        >
      </FaqItem>
      <FaqItem>
        <template #question
          >動画を YouTube
          に投稿せずに、そのままコンテンツ投稿システムにアップロードできますか？</template
        >
        <template #answer
          >いいえ。動画は必ず YouTube
          にアップロードした上で、コンテンツ投稿システム上に埋め込んで下さい。
          動画ファイルは容量が大きいため、YouTubeの基盤を利用することで安定した配信が望めます。
        </template>
      </FaqItem>
    </section>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue-demi'
import { paths } from '@/const/config'
import Breadcrumbs from '@/components/Breadcrumbs.vue'
import Title from './Title.vue'
import FaqItem from './FaqItem.vue'

export default defineComponent({
  components: {
    Breadcrumbs,
    Title,
    FaqItem,
  },
  setup() {
    const navigations = ref([
      { label: paths.top.label(), path: paths.top.path() },
      { label: paths.faq.label(), path: paths.faq.path() },
    ])
    return {
      navigations,
    }
  },
})
</script>

<style lang="scss" scoped>
.faq-item {
  margin: 4rem 0;
}
</style>
