<template>
  <div class="faq-item">
    <div class="question box">
      <div class="cap cap--sky">Q</div>
      <slot name="question" />
    </div>
    <div class="answer box">
      <div class="cap cap--orange">A</div>
      <slot name="answer" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue-demi'

export default defineComponent({})
</script>

<style lang="scss" scoped>
.faq-item {
  background-color: $c-white;
  padding: 2rem;
  border-radius: 0.4rem;
}
.box {
  display: flex;
  margin: 1rem 0rem;
  align-items: center;
  line-height: 1.7;
}
.question {
  margin-bottom: 2rem;
}
.cap {
  @include fs-2;
  flex: 0 0 4rem;
  text-align: center;
  &--sky {
    color: $c-sohosai-sky;
  }
  &--orange {
    color: $c-sohosai-orange;
  }
}
</style>
