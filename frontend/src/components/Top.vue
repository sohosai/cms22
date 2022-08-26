<template>
  <div class="top">
    <div class="left">
      <div class="title">雙峰祭コンテンツ投稿システム</div>
      <div class="message">
        雙峰祭 HP に公開する記事を作成したり
        <br />雙峰祭 HP に掲載してほしいリンクを申請できます。
      </div>
      <div class="start">
        <Button @click="handleStartButton" text="はじめる" />
      </div>
    </div>
    <div class="hero">
      <img src="@/assets/images/topview.svg" />
    </div>
  </div>
</template>

<script lang="ts">
import { paths } from '@/const/config'
import { defineComponent } from 'vue-demi'
import { useRouter } from 'vue-router'
import Button from './Button.vue'

export default defineComponent({
  components: {
    Button,
  },
  setup() {
    const router = useRouter()
    const handleStartButton = () => {
      router.push(paths.contents.path())
    }
    return {
      handleStartButton,
    }
  },
})
</script>

<style lang="scss" scoped>
.top {
  @include center;
  width: 100%;
  height: 100vh;
  background-color: $c-base;
}
.title {
  @include fs-3b;
  padding: 2rem 0;
}
.message {
  line-height: 1.7;
}
.hero {
  width: clamp(1px, 40rem, 50%);
}
.start {
  padding-top: 3rem;
}
</style>
