<template>
  <div class="faq">
    <Breadcrumbs :navigations="navigations" />
    <Title title="よくある質問" />
    <section class="body">
      <FaqItem>
        <template #question>確認メールが届きません。</template>
        <template #answer
          >入力いただいたメールアドレスを再度確認した上で、迷惑メールフォルダもご確認下さい。</template
        >
      </FaqItem>
      <FaqItem>
        <template #question
          >動画を YouTube
          に投稿せずに、そのままコンテンツ投稿システムにアップロードできますか？</template
        >
        <template #answer
          >いいえ。動画は必ず YouTube
          にアップロードした上で、コンテンツ投稿システム上に埋め込んで下さい。このような手順を踏んで頂く理由は、動画ファイルは容量が大きく、安定した配信を行うために
          YouTube を利用するのが最適だからです。</template
        >
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
