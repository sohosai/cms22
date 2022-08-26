<template>
  <div class="preview-image">
    <!-- TODO: File Uploader 部分を別コンポーネントに切り出すべき  -->
    <label for="input" class="label">画像を選択 </label>
    <input
      id="input"
      class="input"
      ref="fileInput"
      type="file"
      @input="pickFile"
    />
    <p class="note">
      ※サムネイルはデバイスによって、以下のように切り取られて表示されます。
    </p>
    <div class="previews">
      <div
        class="preview preview-1"
        :style="{ backgroundImage: `url(${previewImage})` }"
      >
        1:1.91
      </div>
      <div
        class="preview preview-2"
        :style="{ backgroundImage: `url(${previewImage})` }"
      >
        3:2
      </div>
      <div
        class="preview preview-3"
        :style="{ backgroundImage: `url(${previewImage})` }"
      >
        1:1
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue-demi'

export default defineComponent({
  props: {
    image: {
      type: String,
      required: true,
    },
  },
  setup(props, { emit }) {
    const previewImage = ref<string | ArrayBuffer>(props.image)
    const fileInput = ref<HTMLInputElement>()
    const pickFile = () => {
      if (!fileInput.value) return
      let input = fileInput.value
      let file = input.files
      if (file?.[0]) {
        let reader = new FileReader()
        reader.onload = (e) => {
          if (!e.target?.result) return
          previewImage.value = e.target.result
        }
        reader.readAsDataURL(file[0])
        emit('inputImage', file[0])
      }
    }

    return {
      previewImage,
      fileInput,
      pickFile,
    }
  },
})
</script>

<style lang="scss" scoped>
.label {
  @include clickable;
  display: inline-block;
  padding: 1rem 2rem;
  border-radius: 0.8rem;
  background-color: $c-main;
  line-height: 1;
  color: $c-white;
}
.input {
  display: none;
}
.previews {
  display: flex;
  flex-wrap: wrap;
  gap: 2rem;
}
.preview {
  $h: 15rem;
  height: $h;
  border-radius: 0.4rem;
  border: 1px solid $c-gray-dark;
  background-size: cover;
  background-position: center center;
  color: $c-gray;
  &-1 {
    width: $h * 1.91;
  }
  &-2 {
    width: $h * 3 / 2;
  }
  &-3 {
    width: $h;
  }
}
.note {
  @include fs-0;
  margin: 0.5rem 0;
}
</style>
