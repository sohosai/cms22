<template>
  <div class="card-article">
    <div class="title">
      {{ `[${article.project_code}] ${article.project_name}` }}
    </div>
    <div class="sup status">
      <div class="label">状態</div>
      <div>
        <span>{{ articleStatus }}</span>
        <HintTip>
          状態はそれぞれ学園祭実行委員会(総務局)による審査の進行状況を表します。
          <div class="heading">未提出</div>
          この企画にはコンテンツが提出されたことがないので、審査の対象ではありません。
          <div class="heading">審査待ち</div>
          コンテンツが提出され、審査を待っています。
          <div class="heading">承認済み</div>
          コンテンツの掲載が認められました。雙峰祭当日にコンテンツが公開されます。
          <div class="heading">却下</div>
          コンテンツの掲載が却下されました。
        </HintTip>
      </div>
    </div>
    <div class="sup category">
      <div class="label">企画区分</div>
      <div>
        {{ article.project_category }}
      </div>
    </div>
    <div class="sup update-at">
      <div class="label">最終更新日</div>
      <div>{{ article.updated_at.toLocaleString() }}</div>
    </div>
    <div class="action">
      <Button
        @click="handleClick"
        :text="action + '→'"
        width="5rem"
        color="secondary"
      />
    </div>
  </div>
</template>

<script lang="ts">
import { Article } from '@/types/type'
import { contentState } from '@/const/config'
import { defineComponent, PropType } from 'vue'
import Button from './Button.vue'
import HintTip from './HintTip.vue'

type Props = {
  article: Article
}

export default defineComponent({
  components: {
    Button,
    HintTip,
  },
  props: {
    article: {
      type: Object as PropType<Article>,
      required: true,
    },
    action: {
      type: String,
      required: true,
    },
  },
  emits: ['click'],
  setup(props: Props, context) {
    const articleStatus = contentState.find(
      (v) => v.value === props.article.status
    )?.label
    const handleClick = () => {
      context.emit('click', props.article.project_code)
    }

    return {
      articleStatus,
      handleClick,
    }
  },
})
</script>

<style lang="scss" scoped>
.card-article {
  display: grid;
  grid-template:
    'title     ... action' 1fr
    '...       ... action' 2rem
    'status    ... action' 1.6rem
    '...       ... action' 1rem
    'update-at ... action' 1.6rem
    '...       ... action' 1rem
    'cate      ... action' 1.6rem
    / auto 1rem 9rem;
  width: clamp(1px, 70rem, 100%);
  padding: 2rem;
  border-radius: 0.8rem;
  border: 1px solid $c-gray;
  background-color: $c-white;
}
.title {
  @include fs-1b;
  grid-area: title;
}
.sup {
  @include center;
  justify-content: space-between;
}
.action {
  @include center;
  grid-area: action;
}
.status {
  grid-area: status;
}
.category {
  grid-area: cate;
}
.update-at {
  grid-area: update-at;
  text-align: right;
}
.label {
  opacity: 0.7;
  color: $c-text-sub;
  padding-right: 1rem;
}
</style>
