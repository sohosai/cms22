<template>
  <div class="create-article">
    <Breadcrumbs :navigations="navigations" />

    <div class="wrap">
      <div class="heading project-name">
        {{ `[${article.project_code}] ${article.project_name}` }}
      </div>
    </div>

    <div class="wrap">
      <div class="heading">掲載タイプ</div>
      <Select v-model="contentType" :options="contentTypeOptions" />
    </div>

    <div class="wrap content-wrap">
      <template v-if="contentType == 'ArticleContent'">
        <div class="heading">記事本文</div>
        <QuillEditor
          theme="snow"
          toolbar="full"
          v-model:content="contentHtml"
          contentType="html"
        />
      </template>
      <template v-else-if="contentType == 'LinkContent'">
        <div class="heading">リンク先URL</div>
        <FieldText v-model="contentUrl" />
      </template>
    </div>

    <div class="wrap title-wrap">
      <div class="heading">
        サムネイル
        <HintTip>企画一覧ページで利用することがあります。</HintTip>
      </div>
      <PreviewImage @inputImage="handleInputImage" :image="thumbnailUrl" />
    </div>

    <div class="wrap">
      <div v-if="saveAnnotation !== ''" class="deadline-note">
        {{ saveAnnotation }}
      </div>
      <Button
        v-if="saveable"
        @click="handlePostClick"
        :text="saveButtonText"
        :loading="saving"
      />
    </div>
  </div>
</template>

<script lang="ts">
import '@vueup/vue-quill/dist/vue-quill.snow.css'
import { paths } from '@/const/config'
import { defineComponent, onMounted, ref } from 'vue'
import { getUser } from '@/utls/getUser'
import { promiseTimeout } from '@vueuse/core'
import { QuillEditor } from '@vueup/vue-quill'
import { updateContent } from '@/utls/updateContent'
import { getContentByPj } from '@/utls/getContentByPj'
import { useRoute } from 'vue-router'
import { useToggle } from '@vueuse/core'
import Breadcrumbs from './Breadcrumbs.vue'
import Button from './Button.vue'
import FieldText from '@/components/FieldText.vue'
import HintTip from './HintTip.vue'
import PreviewImage from '@/components/PreviewImage.vue'
import { checkSaveAbility } from '@/utls/checkSaveAbility'
import Select from './Select.vue'

export default defineComponent({
  components: {
    Breadcrumbs,
    Button,
    FieldText,
    HintTip,
    QuillEditor,
    PreviewImage,
    Select,
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
    const projectCode = String(route.params.id)
    const [applied, toggleApplied] = useToggle(false)
    const user = getUser()
    if (!user) return

    const saveButtonText = ref('保存する')
    const article = await getContentByPj(projectCode)

    const contentHtml = ref(article.content_html)
    const contentUrl = ref(article.content_url)
    const contentType = ref(article.content_type)

    const contentTypeOptions = [
      {
        value: 'LinkContent',
        label:
          'リンク - ご自身でご用意いただいた外部サイトへのリンクを掲載します',
      },
      {
        value: 'ArticleContent',
        label: '記事 - 本システム上で執筆した記事を掲載します',
      },
    ]

    let thumbnail = new Blob()
    const thumbnailUrl = article.thumbnail

    const navigations = ref([
      { label: paths.contents.label(), path: paths.contents.path() },
      {
        label: paths.editArticle.label(
          `[${article.project_code}] ${article.project_name}`
        ),
        path: paths.editArticle.path(article.project_code),
      },
    ])

    const saveAbility = await checkSaveAbility(article.project_code)
    let saveAnnotation = ''
    let saveable = saveAbility.total

    if (saveAbility.is_committee) {
      saveAnnotation =
        '実委人は期限や設定に関わらずいつでも記事を編集・保存できます。この機能は注意して使用してください。'
    } else if (!saveAbility.is_my_project) {
      saveAnnotation = '自分の企画ではないため保存できません。' // そもそもgetでこけるはずなのでunreachableなはず
    } else if (!saveAbility.in_save_period && saveAbility.is_editable) {
      saveAnnotation =
        '申請期間外ですが、この企画には例外的な保存許可が設定されています。'
    }

    const saving = ref(false)
    const handleInputImage = (file: Blob) => {
      thumbnail = file
    }
    const handlePostClick = async () => {
      saving.value = true
      saveButtonText.value = '保存中'

      const newArticle = {
        ...article,
        content_type: contentType.value,
        content_html: contentHtml.value,
        content_url: contentUrl.value,
      }

      console.log(thumbnail)
      await updateContent(newArticle, thumbnail)

      saving.value = false
      saveButtonText.value = '保存完了'
      await promiseTimeout(2000)
      saveButtonText.value = '保存する'
    }

    return {
      applied,
      article,
      contentType,
      contentHtml,
      contentUrl,
      handlePostClick,
      navigations,
      saveButtonText,
      saving,
      thumbnailUrl,
      handleInputImage,
      toggleApplied,
      saveAnnotation,
      saveable,
      contentTypeOptions,
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

.project-name {
  font-size: 1.5em;
  font-weight: bold;
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
