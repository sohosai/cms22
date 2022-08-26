<template>
  <div class="signin">
    <div id="firebaseui-auth-container"></div>
    <div class="grand">
      <div class="inner">
        ※「tsukuba.ac.jp」で終わるメールアドレスをご使用ください。
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted } from 'vue'
import firebase from 'firebase'
import * as firebaseui from 'firebaseui'
import 'firebase/auth'
import 'firebaseui/dist/firebaseui.css'
import { paths } from '@/const/config'

export default defineComponent({
  components: {},
  setup() {
    onMounted(() => {
      const ui = new firebaseui.auth.AuthUI(firebase.auth())
      ui.start('#firebaseui-auth-container', {
        signInOptions: [firebase.auth.EmailAuthProvider.PROVIDER_ID],
        signInSuccessUrl: paths.contents.path(),
        callbacks: {
          signInSuccessWithAuthResult: (authResult) => {
            const user = authResult.user
            if (authResult.additionalUserInfo.isNewUser) {
              user.sendEmailVerification()
            }
            return true
          },
        },
      })
    })
  },
})
</script>

<style lang="scss" scoped>
.signin {
  @include center;
  width: 100%;
  height: 100vh;
  padding-bottom: 20vh;
  background-color: $c-base;
}
#firebaseui-auth-container {
  width: 100%;
}
.grand {
  @include center;
  position: absolute;
  top: 40vh;
  width: 100%;
  width: clamp(25rem, 40vw, 50rem);
  height: clamp(25rem, 40vw, 50rem);
  border-radius: 100vw;
  background-color: $c-main;
}
.inner {
  padding: 10%;
  color: $c-white;
}
</style>
