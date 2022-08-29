<template>
  <div class="sidebar">
    <div class="main">
      <button class="title" @click="$router.push('/')">
        雙峰祭コンテンツ
        <br />投稿システム
      </button>
      <div class="account">
        <template v-if="user">
          <div>
            <div class="account_item display-name">{{ displayName }}</div>
            <div class="account_item email">{{ user.email }}</div>
          </div>
          <span class="material-icons" @click="signout">logout</span>
        </template>
        <button class="login" v-else @click="$router.push(paths.signin.path())">
          サインイン
          <span class="material-icons">login</span>
        </button>
      </div>
      <nav v-if="isAuditor">
        <button
          v-for="item in auditorMenu"
          :key="item.label"
          class="nav-item"
          @click="$router.push(item.link)"
        >
          {{ item.label }}
        </button>
      </nav>
      <nav v-else>
        <button
          v-for="item in menu"
          :key="item.label"
          class="nav-item"
          @click="$router.push(item.link)"
        >
          {{ item.label }}
        </button>
      </nav>
    </div>
    <div class="slot">
      <slot />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import firebase from 'firebase'
import 'firebase/auth'
import { useRouter } from 'vue-router'
import { paths } from '@/const/config'
import { getUser } from '@/utls/getUser'
import { getMyProfile } from '@/utls/getMyProfile'

export default defineComponent({
  async setup() {
    const router = useRouter()
    const menu = [
      { label: paths.contents.label(), link: paths.contents.path() },
      { label: paths.faq.label(), link: paths.faq.path() },
      { label: paths.contact.label(), link: paths.contact.path() },
    ]
    const auditorMenu = [
      { label: paths.contents.label(), link: paths.contents.path() },
      {
        label: paths.postedContents.label(),
        link: paths.postedContents.path(),
      },
      { label: paths.faq.label(), link: paths.faq.path() },
      { label: paths.contact.label(), link: paths.contact.path() },
    ]
    const user = getUser()
    const me = await getMyProfile()
    const isAuditor = me.is_committee
    const displayName = me.name
    const signout = async () => {
      firebase.auth().onIdTokenChanged(async () => {
        try {
          await firebase.auth().signOut()
          router.push(paths.signin.path())
        } catch (e) {
          alert('サインアウトに失敗しました。')
        }
      })
    }

    return {
      menu,
      auditorMenu,
      signout,
      user,
      paths,
      isAuditor,
      displayName,
    }
  },
})
</script>

<style lang="scss" scoped>
.sidebar {
  display: flex;
  justify-content: flex-start;
  align-items: flex-start;
  width: 100%;
  background-color: $c-base;
}
.main {
  flex: 0 0 20rem;
  height: 100vh;
  box-shadow: 0.3rem 0 0.6rem $c-shadow;
  background-color: $c-main-dark;
  color: $c-gray;
}
.slot {
  width: 100%;
  height: 100vh;
  padding: 0 2rem 10rem;
  overflow-y: scroll;
}
.title {
  @include fs-1b;
  padding: 2rem 1rem;
}
.account {
  @include center;
  gap: 0 1rem;
  margin-bottom: 3rem;
  padding: 1rem;
  background-color: $c-main;
  &_item {
    padding: 0.5rem 0;
  }
}
.email-verified-alert {
  color: $c-error;
}
.login {
  @include center;
  gap: 0 1rem;
}
.display-name {
  @include fs-1b;
}
.email {
  @include fs-0;
  color: $c-gray;
}
nav {
  @include fs-1;
}
.nav-item {
  width: 100%;
  padding: 1rem;
  &:hover {
    background-color: $c-main-dark2;
  }
}
</style>
