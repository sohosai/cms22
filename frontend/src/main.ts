import '@/assets/css/main.scss'
import { createApp } from 'vue'
import App from './App.vue'
import firebase from 'firebase/app'
import router from './router'
import { createGtm } from '@gtm-support/vue-gtm'

const firebaseConfig = {
  apiKey: process.env.VUE_APP_FIREBASE_API_KEY,
  authDomain: process.env.VUE_APP_FIREBASE_AUTH_DOMAIN,
  projectId: process.env.VUE_APP_FIREBASE_PROJECT_ID,
  storageBucket: process.env.VUE_APP_FIREBASE_STORAGE_BUCKET,
  messagingSenderId: process.env.VUE_APP_FIREBASE_MESSAGING_SENDER_ID,
  appId: process.env.VUE_APP_FIREBASE_APP_ID,
}

firebase.initializeApp(firebaseConfig)

if (
  location.hostname === 'localhost' &&
  process.env.VUE_APP_FIREBASE_EMULATORS !== 'disable'
) {
  firebase.auth().useEmulator('http://localhost:9099')
  firebase.firestore().useEmulator('localhost', 8080)
  firebase.functions().useEmulator('localhost', 5001)
  firebase.storage().useEmulator('localhost', 9199)
}

createApp(App)
  .use(router)
  .use(
    createGtm({
      id: 'GTM-N2KQDG4',
    })
  )
  .mount('#app')
