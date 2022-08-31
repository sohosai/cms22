<template>
  <div class="posted-contents">
    <Breadcrumbs :navigations="navigations" />
    <Title title="コンテンツ一覧" />
    <div class="settings">
      <div class="settings_title">表示設定</div>
      <Checkbox
        v-model="showNotPending"
        label="現在審査待ちではないコンテンツも表示"
      />

      <HintTip>
        これまでに提出されたことがないまたはすでに審査済みなどで、現在審査待ちでない企画も表示します。
      </HintTip>

      <br />
      表示する企画区分の種類
      <template v-for="item in categoryFilter" :key="item.category">
        <br /><Checkbox v-model="item.show" :label="item.category" />
      </template>
    </div>
    <div
      v-for="article in articles"
      :key="article.project_code"
      class="article"
    >
      <template
        v-if="
          categoryFilter.find(
            (item) => item.category === article.project_category
          )?.show &&
          (showNotPending || article.status == 'Pending')
        "
      >
        <CardArticle
          :article="article"
          @click="handleArticleClick"
          action="審査"
        />
      </template>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import { getContents } from '@/utls/getContents'
import { paths } from '@/const/config'
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
    const articles = await getContents()
    const handleArticleClick = (projectCode: string) => {
      router.push(paths.auditArticle.path(projectCode))
    }

    const categories = [
      'オンライン一般企画',
      '対面一般企画',
      'オンラインステージ企画',
      '対面ステージ企画',
      '調理企画',
      '飲食物取扱い企画',
      '本部企画',
    ]
    const showNotPending = ref(false)

    const categoryFilter = ref(
      categories.map((category) => ({
        category,
        show: true,
      }))
    )
    return {
      articles,
      handleArticleClick,
      navigations,
      showNotPending,
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
