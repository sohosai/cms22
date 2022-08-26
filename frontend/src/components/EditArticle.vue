<template>
  <div class="create-article">
    <Breadcrumbs :navigations="navigations" />
    <div class="wrap">
      <div class="heading">
        企画区分
        <HintTip
          >応募した企画の種類を選択してください。どれにも当てはまらない方は「その他」を選択してください。</HintTip
        >
      </div>
      <Select v-model="category" :options="contentCategory" />
      <div
        v-if="contentCategory.find((v) => v.value === category)?.deadline"
        class="deadline-note"
      >
        <p>
          {{
            contentCategory.find((v) => v.value === category)?.label
          }}の申請期限は
          <span class="strong">
            {{
              contentCategory
                .find((v) => v.value === category)
                ?.deadline.toLocaleString()
            }}
          </span>
          です。この期限以降は記事を編集できなくなります。この期限以内に[保存する]ボタンを押して、コンテンツを保存してください。
        </p>
      </div>
      <Checkbox label="この企画は学術企画に属する" v-model="academic" />
      <HintTip
        >応募した企画が学術企画に属する場合はチェックを入れてください。</HintTip
      >
    </div>
    <div class="wrap">
      <div class="heading">
        企画名
        <HintTip>応募した企画の名前を入力してください。</HintTip>
      </div>
      <FieldText v-model="projectName" />
    </div>
    <div class="wrap title-wrap">
      <div class="heading">
        <template v-if="articleType == 'html'">
          <div class="heading">
            記事タイトル
            <HintTip>長い場合は見切れる可能性があります。</HintTip>
          </div>
        </template>
        <template v-else-if="articleType == 'url'">
          <div class="heading">
            リンク文字
            <HintTip
              >リンクで表示される文字です。長い場合は見切れる可能性があります。</HintTip
            >
          </div>
        </template>
      </div>
      <FieldText v-model="title" />
    </div>
    <div v-if="category.startsWith('headquarters')" class="wrap title-wrap">
      <div class="heading">
        記事サムネイル
        <HintTip>本部企画一覧ページで表示されます。</HintTip>
      </div>
      <PreviewImage @inputImage="handleInputImage" :image="decodedThumbnail" />
    </div>
    <div class="wrap content-wrap">
      <template v-if="articleType == 'html'">
        <div class="heading">記事本文</div>
        <QuillEditor
          theme="snow"
          toolbar="full"
          v-model:content="contentHtml"
          contentType="html"
        />
      </template>
      <template v-else-if="articleType == 'url'">
        <div class="heading">リンク先URL</div>
        <FieldText v-model="url" />
      </template>
    </div>
    <div class="wrap">
      <div
        v-if="
          contentCategory.find((v) => v.value === category)?.deadline <
          new Date()
        "
        class="deadline-note"
      >
        <p>
          <span class="strong"
            >{{
              contentCategory.find((v) => v.value === category)?.label
            }}の申請期限を過ぎているため保存できません。</span
          >
        </p>
      </div>
      <Button
        v-else
        @click="handlePostClick"
        :text="saveButtonText"
        :loading="saving"
      />
    </div>
    <div class="wrap">
      <Button
        @click="handleDeleteClick"
        :text="deleteButtonText"
        :loading="deleting"
      />
    </div>
  </div>
</template>

<script lang="ts">
import '@vueup/vue-quill/dist/vue-quill.snow.css'
import 'firebase/firestore'
import { contentCategory, paths } from '@/const/config'
import { convBlobToBase64 } from '@/utls/convBlobToBase64'
import { defineComponent, onMounted, ref } from 'vue'
import { deleteArticle } from '@/utls/deleteArticle'
import { getThumbnailByArticle } from '@/utls/getThumbnailByArticle'
import { getUser } from '@/utls/getUser'
import { isURLArticle } from '@/types/type'
import { promiseTimeout } from '@vueuse/core'
import { QuillEditor } from '@vueup/vue-quill'
import { updateArticleByUserAndId } from '@/utls/updateArticleByUserAndId'
import { useRoute, useRouter } from 'vue-router'
import { useToggle } from '@vueuse/core'
import Breadcrumbs from './Breadcrumbs.vue'
import Button from './Button.vue'
import Checkbox from '@/components/Checkbox.vue'
import FieldText from '@/components/FieldText.vue'
import HintTip from './HintTip.vue'
import PreviewImage from '@/components/PreviewImage.vue'
import Select from '@/components/Select.vue'
import { getArticleByUserAndId } from '@/utls/getArticleByUserAndId'

export default defineComponent({
  components: {
    Breadcrumbs,
    Button,
    Checkbox,
    FieldText,
    HintTip,
    QuillEditor,
    Select,
    PreviewImage,
  },
  async setup() {
    onMounted(() => {
      const editorTooltipItems = [
        {
          querySelector: '.ql-bold',
          toolTip: '太字',
        },
        {
          querySelector: '.ql-italic',
          toolTip: '斜体',
        },
        {
          querySelector: '.ql-underline',
          toolTip: '下線',
        },
        {
          querySelector: '.ql-strike',
          toolTip: '打ち消し線',
        },
        {
          querySelector: '.ql-blockquote',
          toolTip: '引用ブロック',
        },
        {
          querySelector: '.ql-code-block',
          toolTip: 'コードブロック',
        },
        {
          querySelector: '.ql-header[value="1"]',
          toolTip: '見出し1',
        },
        {
          querySelector: '.ql-header[value="2"]',
          toolTip: '見出し2',
        },
        {
          querySelector: '.ql-list[value="ordered"]',
          toolTip: '順序つきリスト',
        },
        {
          querySelector: '.ql-list[value="ordered"]',
          toolTip: '順序なしリスト',
        },
        {
          querySelector: '.ql-list[value="bullet"]',
          toolTip: '順序なしリスト',
        },
        {
          querySelector: '.ql-script[value="sub"]',
          toolTip: '下付き文字',
        },
        {
          querySelector: '.ql-script[value="super"]',
          toolTip: '上付き文字',
        },
        {
          querySelector: '.ql-indent[value="-1"]',
          toolTip: '字下げ解除',
        },
        {
          querySelector: '.ql-indent[value="+1"]',
          toolTip: '字下げ',
        },
        {
          querySelector: '.ql-size.ql-picker',
          toolTip: '文字の大きさ',
        },
        {
          querySelector: '.ql-header.ql-picker',
          toolTip: '見出しレベル',
        },
        {
          querySelector: '.ql-color',
          toolTip: '文字色',
        },
        {
          querySelector: '.ql-background',
          toolTip: '背景色',
        },
        {
          querySelector: '.ql-background',
          toolTip: '文字背景色',
        },

        {
          querySelector: '.ql-font',
          toolTip: '書体',
        },
        {
          querySelector: '.ql-align',
          toolTip: '揃え方向',
        },
        {
          querySelector: '.ql-link',
          toolTip: 'リンクを挿入',
        },
        {
          querySelector: '.ql-video',
          toolTip: '動画を挿入',
        },
        {
          querySelector: '.ql-image',
          toolTip: '画像を挿入',
        },
        {
          querySelector: '.ql-clean',
          toolTip: '文字装飾なしに戻す',
        },
      ]

      for (const toolTipItem of editorTooltipItems) {
        const element: HTMLButtonElement | null = document.querySelector(
          toolTipItem.querySelector
        )
        if (element) {
          element.title = toolTipItem.toolTip
        }
      }
    })

    const route = useRoute()
    const router = useRouter()
    const articleId = String(route.params.id)
    const [applied, toggleApplied] = useToggle(false)
    const user = getUser()
    if (!user) return
    const saveButtonText = ref('保存する')
    const deleteButtonText = ref('このコンテンツを削除する')
    const article = await getArticleByUserAndId(user.uid, articleId)
    const title = ref(article.title)
    const projectName = ref(article.projectName)
    const contentHtml = isURLArticle(article)
      ? ref('')
      : ref(article.contentHtml)
    const url = isURLArticle(article) ? ref(article.url) : ref('')
    const category = ref(article.category)
    const academic = ref(article.academic)
    const articleType = ref(article.articleType)
    let thumbnail = new Blob()
    const decodedThumbnail = article.category.startsWith('headquarters')
      ? await convBlobToBase64(await getThumbnailByArticle(article))
      : ''
    const navigations = ref([
      { label: paths.contents.label(), path: paths.contents.path() },
      {
        label: paths.editArticle.label(article.title),
        path: paths.editArticle.path(articleId),
      },
    ])
    const saving = ref(false)
    const deleting = ref(false)
    const handleInputImage = (file: Blob) => {
      thumbnail = file
    }
    const handlePostClick = async () => {
      if (category.value === 'unselected') {
        alert('企画区分を選択してください。')
        return
      }
      if (projectName.value === '') {
        alert('企画名を入力してください。')
        return
      }
      saving.value = true
      saveButtonText.value = '保存中'

      if (isURLArticle(article)) {
        const newArticle = {
          ...article,
          projectName: projectName.value,
          title: title.value,
          category: category.value,
          academic: academic.value,
          url: url.value,
        }
        await updateArticleByUserAndId(
          newArticle,
          thumbnail.size > 0 ? thumbnail : undefined
        )
      } else {
        const newArticle = {
          ...article,
          projectName: projectName.value,
          title: title.value,
          category: category.value,
          academic: academic.value,
          contentHtml: contentHtml.value,
        }
        await updateArticleByUserAndId(
          newArticle,
          thumbnail.size > 0 ? thumbnail : undefined
        )
      }
      saving.value = false
      saveButtonText.value = '保存完了'
      await promiseTimeout(2000)
      saveButtonText.value = '保存する'
    }
    const handleDeleteClick = async () => {
      if (!confirm('このコンテンツを削除します。よろしいですか？')) {
        return
      }
      deleting.value = true
      deleteButtonText.value = '削除中'
      await deleteArticle(articleId, article.authorId)
      router.push(paths.contents.path())
    }

    return {
      academic,
      applied,
      article,
      articleType,
      category,
      contentCategory,
      contentHtml,
      deleteButtonText,
      deleting,
      handleDeleteClick,
      handlePostClick,
      navigations,
      projectName,
      saveButtonText,
      saving,
      decodedThumbnail,
      handleInputImage,
      title,
      toggleApplied,
      url,
    }
  },
})
</script>

<style lang="scss" scoped>
.wrap {
  width: clamp(1px, 100rem, 100%);
  margin-bottom: 4rem;
}
.heading {
  margin-bottom: 1rem;
}
.content-wrap {
  ::v-deep(.ql-toolbar) {
    border-color: $c-gray-dark;
    border-top-right-radius: 0.4rem;
    border-top-left-radius: 0.4rem;
  }
  ::v-deep(.ql-container) {
    border-color: $c-gray-dark;
    @include fs-1;
    border-bottom-right-radius: 0.4rem;
    border-bottom-left-radius: 0.4rem;
  }
  ::v-deep(.ql-editor) {
    line-height: 1.7;
  }
  ::v-deep(img) {
    text-align: center;
    max-width: 100%;
  }
  ::v-deep(iframe) {
    width: clamp(1px, 60rem, 100%);
    height: 60rem / 16 * 9;
  }
}

.deadline-note {
  padding: 1rem 0;
  .strong {
    color: red;
  }
}
</style>
