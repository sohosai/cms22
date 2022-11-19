<template>
  <div class="top">
    <div class="left">
      <div class="title">雙峰祭コンテンツ投稿システム</div>
      <div class="message">
        <p>本システムは、11/19をもって提供を終了いたしました。</p>
        <p>雙峰祭へのご参加ありがとうございました。</p>
        <p>提出いただいたコンテンツは、<a href="https://sohosai.com/">雙峰祭公式ウェブサイト</a>にて11月末まで公開いたします。</p>
      </div>
    </div>
    <div class="hero">
      <img src="@/assets/images/topview.svg" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue-demi'

export default defineComponent({
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
  width: 30vw;
}
.hero {
  width: clamp(1px, 40rem, 50%);
}
.start {
  padding-top: 3rem;
}
</style>
