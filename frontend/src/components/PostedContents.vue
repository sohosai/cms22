<template>
  <div class="posted-contents">
    <Breadcrumbs :navigations="navigations" />
    <Title title="コンテンツ一覧" />
    <div class="settings">
      <div class="settings_title">表示設定</div>
      <Checkbox
        v-model="showUnAuditable"
        label="現在審査可能ではないコンテンツも表示"
      />
      <HintTip>
        編集可能期間中の記事など、現在審査できない記事も表示します。
      </HintTip>
      <br />
      <Checkbox v-model="showAudited" label="承認済みのコンテンツも表示" />
      <br />
      <br />
      表示する企画区分の種類
      <template v-for="category in categoryFilter" :key="category.value">
        <br /><Checkbox v-model="category.show" :label="category.label" />
      </template>
    </div>
    <div v-for="article in articles" :key="article.title" class="article">
      <template
        v-if="
          (contentCategory.find((v) => v.value === article?.category)
            ?.deadline < new Date() ||
            showUnAuditable ||
            article.category.endsWith('auditor')) &&
          (article.state !== 'verified' || showAudited) &&
          categoryFilter.find((v) => v.category === article.category)?.show
        "
      >
        <CardArticle
          :article="article"
          @click="handleArticleClick"
          action="検閲"
        />
      </template>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import { getContents } from '@/utls/getContents'
import { contentCategory, paths } from '@/const/config'
import { useRouter } from 'vue-router'
import Breadcrumbs from '@/components/Breadcrumbs.vue'
import CardArticle from './CardArticle.vue'
import Checkbox from '@/components/Checkbox.vue'
import Title from './Title.vue'
import HintTip from '@/components/HintTip.vue'

export default defineComponent({
  components: {
    Breadcrumbs,
    CardArticle,
    Checkbox,
    Title,
    HintTip,
  },
  async setup() {
    const router = useRouter()
    const navigations = ref([
      { label: paths.top.label(), path: paths.top.path() },
      {
        label: paths.postedContents.label(),
        path: paths.postedContents.path(),
      },
    ])
    const { articles } = await getContents()
    const handleArticleClick = (articleId: string) => {
      router.push(paths.auditArticle.path(articleId))
    }
    const showUnAuditable = ref(false)
    const showAudited = ref(false)
    const categoryFilter = ref(
      contentCategory.map((category) => ({
        category: category.value,
        label: category.label,
        show: true,
      }))
    )
    return {
      articles,
      contentCategory,
      handleArticleClick,
      navigations,
      showAudited,
      showUnAuditable,
      categoryFilter,
    }
  },
})
</script>

<style lang="scss" scoped>
.settings {
  margin: 2rem 0;
  padding: 2rem 0;
  border-top: 1px solid $c-gray-dark;
  border-bottom: 1px solid $c-gray-dark;
  &_title {
    margin-bottom: 2rem;
  }
}
.article {
  margin: 1rem 0;
}
</style>
