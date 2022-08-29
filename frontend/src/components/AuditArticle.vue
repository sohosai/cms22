<template>
  <div class="audit-article">
    <Breadcrumbs :navigations="navigations" />

    <Button
      v-if="article.status === 'Approved'"
      @click="handleAuditClick('Pending')"
      text="承認を取り消す"
      :loading="saving"
    />
    <Button
      v-else-if="article.status === 'Rejected'"
      @click="handleAuditClick('Pending')"
      text="却下を取り消す"
      :loading="saving"
    />
    <Button
      v-else-if="article.status === 'NeverSubmitted'"
      disabled
      text="まだ一度も提出されていません"
      :loading="saving"
    />
    <div v-else>
      <Button
        @click="handleAuditClick('Approved')"
        text="承認する"
        :loading="saving"
      />

      <Button
        @click="handleAuditClick('Rejected')"
        text="却下する"
        :loading="saving"
      />
    </div>

    <br />
    <Button @click="handleEditClick" text="この記事を管理者として編集する" />
    <div class="section-title">企画内容</div>
    <div class="row">
      <div class="item">企画名</div>
      <div class="value">
        {{ article.project_name }}
      </div>
    </div>
    <div class="row">
      <div class="item">企画区分</div>
      <div class="value">
        {{ article.project_category }}
      </div>
    </div>
    <div class="row">
      <div class="item">学術企画</div>
      <div class="value">
        {{ article.is_academic ? 'はい' : 'いいえ' }}
      </div>
    </div>
    <div class="row">
      <div class="item">芸祭企画</div>
      <div class="value">
        {{ article.is_art ? 'はい' : 'いいえ' }}
      </div>
    </div>

    <div class="row">
      <div class="item">
        {{ article.content_type === 'ArticleContent' ? '記事本文' : 'URL' }}
      </div>
      <div class="value">
        <div
          v-if="article.content_type === 'ArticleContent'"
          v-html="sanitizedHtml"
          class="article-body"
        ></div>
        <div v-else>
          <a :href="article.content_url">{{ article.content_url }}</a>
        </div>
      </div>
    </div>
    <div class="section-title">企画者情報</div>
    <div class="row">
      <div class="item">責任者</div>
      <div class="value">{{ ownerProfile.name }}</div>
    </div>
    <div class="row">
      <div class="item">責任者連絡先</div>
      <div class="value">{{ ownerProfile.email }}</div>
    </div>
  </div>
</template>

<script lang="ts">
import { paths } from '@/const/config'
import { defineComponent, ref } from 'vue'
import { getContentByPj } from '@/utls/getContentByPj'
import { sanitizeHtml } from '@/utls/sanitizeHtml'
import { updateContentState } from '@/utls/updateContentState'
import { useRoute, useRouter } from 'vue-router'
import { ReviewState } from '@/types/type'
import Breadcrumbs from '@/components/Breadcrumbs.vue'
import Button from '@/components/Button.vue'

export default defineComponent({
  components: {
    Button,
    Breadcrumbs,
  },
  async setup() {
    const route = useRoute()
    const router = useRouter()
    const projectCode = String(route.params.id)
    const article = await getContentByPj(projectCode)
    const sanitizedHtml = sanitizeHtml(article.content_html)

    const navigations = ref([
      {
        label: paths.postedContents.label(),
        path: paths.postedContents.path(),
      },
      {
        label: paths.auditArticle.label(
          `[${article.project_code}] ${article.project_name}`
        ),
        path: paths.auditArticle.path(String(article.project_code)),
      },
    ])
    console.log(article)
    const ownerProfile = article.owner
    const saving = ref(false)
    const handleEditClick = () => {
      router.push(paths.editArticle.path(article.project_code))
    }

    const handleAuditClick = async (state: ReviewState) => {
      saving.value = true
      await updateContentState(article.project_code, state)
      saving.value = false
      window.location.reload()
    }

    return {
      article,
      ownerProfile,
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
