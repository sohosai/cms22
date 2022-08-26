<template>
  <div class="audit-article">
    <Breadcrumbs :navigations="navigations" />
    <div>
      <template
        v-if="
          contentCategory.find((v) => v.value === article.category)?.deadline <
            new Date() || article.category.endsWith('auditor')
        "
      >
        <Button
          v-if="article.state === 'verified'"
          @click="handleAuditClick('editable')"
          text="承認を取り消す"
          :loading="saving"
        />
        <Button
          v-else
          @click="handleAuditClick('verified')"
          text="承認する"
          :loading="saving"
        />
      </template>
      <div v-else>
        この記事はまだ編集可能期間中なため、承認することができません。
        <HintTip>
          編集可能期間中は記事を承認することができません。編集可能中は記事が常に変更される可能性があるためです。
        </HintTip>
      </div>
    </div>
    <br />
    <Button @click="handleEditClick" text="この記事を管理者として編集する" />
    <div class="section-title">企画内容</div>
    <div class="row">
      <div class="item">企画名</div>
      <div class="value">
        {{ article.projectName }}
      </div>
    </div>
    <div class="row">
      <div class="item">企画区分</div>
      <div class="value">
        {{ contentCategory.find((v) => v.value === article.category)?.label }}
      </div>
    </div>
    <div class="row">
      <div class="item">学術企画に属する</div>
      <div class="value">
        {{ article.academic ? 'はい' : 'いいえ' }}
      </div>
    </div>
    <div class="row">
      <div class="item">
        {{ article.articleType === 'html' ? '記事タイトル' : 'リンクタイトル' }}
      </div>
      <div class="value">
        {{ article.title }}
      </div>
    </div>
    <div class="row">
      <div class="item">
        {{ article.articleType === 'html' ? '記事本文' : 'URL' }}
      </div>
      <div class="value">
        <div
          v-if="article.articleType === 'html'"
          v-html="sanitizedHtml"
          class="article-body"
        ></div>
        <div v-else>
          <a :href="article.url">{{ article.url }}</a>
        </div>
      </div>
    </div>
    <div class="section-title">企画者情報</div>
    <div class="row">
      <div class="item">企画者名</div>
      <div class="value">{{ authorProfile.name }}</div>
    </div>
    <div class="row">
      <div class="item">企画者連絡先</div>
      <div class="value">{{ authorProfile.email }}</div>
    </div>
    <div class="row">
      <div class="item">メールが認証済みか</div>
      <div class="value">
        {{ authorProfile.verified ? 'はい' : 'いいえ' }}
        <div v-if="!authorProfile.verified">
          なりすましの可能性があります。メールを認証するよう催促することをおすすめします。
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import 'firebase/firestore'
import { contentCategory, paths } from '@/const/config'
import { ContentState, isURLArticle } from '@/types/type'
import { defineComponent, ref } from 'vue'
import { fetchUserProfile } from '@/utls/fetchUserProfile'
import { getArticleById } from '@/utls/getArticleById'
import { sanitizeHtml } from '@/utls/sanitizeHtml'
import { updateArticleState } from '@/utls/updateArticleState'
import { useRoute, useRouter } from 'vue-router'
import Breadcrumbs from '@/components/Breadcrumbs.vue'
import Button from '@/components/Button.vue'
import HintTip from '@/components/HintTip.vue'

export default defineComponent({
  components: {
    Button,
    HintTip,
    Breadcrumbs,
  },
  async setup() {
    const route = useRoute()
    const router = useRouter()
    const articleId = String(route.params.id)
    const article = await getArticleById(articleId)
    let sanitizedHtml = ''
    if (!isURLArticle(article)) {
      sanitizedHtml = sanitizeHtml(article.contentHtml)
    }
    const navigations = ref([
      {
        label: paths.postedContents.label(),
        path: paths.postedContents.path(),
      },
      {
        label: paths.auditArticle.label(article.title),
        path: paths.auditArticle.path(String(articleId)),
      },
    ])
    console.log(article)
    const res = await fetchUserProfile(article.authorId)
    const authorProfile = res.profile
    const saving = ref(false)
    const handleEditClick = () => {
      router.push(paths.editArticle.path(article.id))
    }
    const handleAuditClick = async (state: ContentState) => {
      saving.value = true
      await updateArticleState(article.authorId, articleId, state)
      saving.value = false
      window.location.reload()
    }
    return {
      article,
      authorProfile,
      contentCategory,
      handleAuditClick,
      handleEditClick,
      navigations,
      sanitizedHtml,
      saving,
    }
  },
})
</script>

<style lang="scss" scoped>
.section-title {
  @include fs-2;
  padding: 3rem 0 1rem;
  border-bottom: 1px solid $c-gray;
}
.row {
  padding: 2rem 0;
}
.item {
  @include fs-0b;
  margin-bottom: 1rem;
  color: $c-text-sub;
}
.value {
  padding-left: 1rem;
}
.article-body {
  background: $c-white;
  padding: 1rem;
  border-radius: 0.5rem;
}
</style>
