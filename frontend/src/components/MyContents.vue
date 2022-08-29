<template>
  <div class="my-contents">
    <Breadcrumbs :navigations="navigations" />
    <Title title="マイコンテンツ" />
    <div class="heading">記事投稿</div>
    <div class="description">
      <p>雙峰祭ウェブサイト上で掲載する企画紹介記事を投稿できます。</p>
    </div>
    <div v-for="article in articles" :key="article.projectCode">
      <CardArticle
        :article="article"
        action="編集"
        @click="handleEditArticle"
      />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import { getContentsByUser } from '@/utls/getContentsByUser'
import { paths } from '@/const/config'
import { useRouter } from 'vue-router'
import Breadcrumbs from '@/components/Breadcrumbs.vue'
import CardArticle from '@/components/CardArticle.vue'
import Title from './Title.vue'
import { getUser } from '@/utls/getUser'

export default defineComponent({
  components: {
    Breadcrumbs,
    CardArticle,
    Title,
  },
  async setup() {
    const router = useRouter()
    const user = getUser()
    if (!user) return

    const navigations = ref([
      { label: paths.top.label(), path: paths.top.path() },
      { label: paths.contents.label(), path: paths.contents.path() },
    ])

    const articles = await getContentsByUser()

    const handleEditArticle = (projectCode: string) => {
      router.push(paths.editArticle.path(projectCode))
    }

    return {
      handleEditArticle,
      navigations,
      articles,
    }
  },
})
</script>

<style lang="scss" scoped>
.heading {
  @include fs-2;
  margin: 2rem 0 1rem;
}
.description {
  margin-bottom: 2rem;
}
.create-button {
  margin-top: 2rem;
}
</style>
