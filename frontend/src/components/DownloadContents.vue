<template>
  <div class="donwload-contents">
    <a :href="donwloadUrl" download="contents..json">ダウンロード</a>
  </div>
</template>

<script lang="ts" setup>
import { getArticleById } from '@/utls/getArticleById'
import { getContents } from '@/utls/getContents'

const { articles: overviews } = await getContents()

console.log(overviews)
const promises = overviews.map(
  async (overview) => await getArticleById(overview.id)
)
const contents = await Promise.all(promises)
console.log(contents)
const json = JSON.stringify(contents, null, '  ')
const blob = new Blob([json], { type: 'application/json' })
const donwloadUrl = URL.createObjectURL(blob)
</script>
