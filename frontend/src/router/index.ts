import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import { paths } from '@/const/config'
import AuditArticle from '@/components/AuditArticle.vue'
import Contact from '@/components/Contact.vue'
import DownloadContents from '@/components/DownloadContents.vue'
import EditArticle from '../components/EditArticle.vue'
import Faq from '@/components/Faq.vue'
import firebase from 'firebase'
import Layout from '../components/Layout.vue'
import MyContents from '../components/MyContents.vue'
import PostedContents from '@/components/PostedContents.vue'
import ReflectAuthority from '@/components/ReflectAuthority.vue'
import Signin from '@/components/Signin.vue'
import Top from '@/components/Top.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: paths.signin.path(),
    component: Signin,
  },
  {
    path: paths.top.path(),
    component: Top,
  },
  {
    path: '/',
    component: Layout,
    children: [
      {
        path: paths.contents.path(),
        component: MyContents,
        meta: { requireAuth: true },
      },
      {
        path: paths.contents.path() + '/:id/edit',
        component: EditArticle,
        meta: { requireAuth: true },
      },
      {
        path: paths.postedContents.path(),
        component: PostedContents,
        meta: { requireAuth: true, requireAuditor: true },
      },
      {
        path: paths.downloadContents.path(),
        component: DownloadContents,
        meta: { requireAuth: true, requireAuditor: true },
      },
      {
        path: paths.contents.path() + '/:id/audit',
        component: AuditArticle,
        meta: { requireAuth: true, requireAuditor: true },
      },
      {
        path: paths.contact.path(),
        component: Contact,
      },
      {
        path: paths.faq.path(),
        component: Faq,
      },
    ],
  },
  {
    path: paths.reflectAuthority.path(),
    component: ReflectAuthority,
  },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
})

router.beforeEach((to, _, next) => {
  const requireAuth = to.matched.some((record) => record.meta.requireAuth)
  const requireAuditor = to.matched.some((record) => record.meta.requireAuditor)
  if (!requireAuth) {
    next()
    return
  }
  firebase.auth().onAuthStateChanged(async (user) => {
    if (!user) {
      next({
        path: paths.signin.path(),
        query: { redirect: to.fullPath },
      })
      return
    }
    if (!requireAuditor) {
      next()
      return
    }
    const result = await user.getIdTokenResult()
    if (result.claims.auditor) {
      next()
    } else {
      next({
        path: paths.signin.path(),
        query: { redirect: to.fullPath },
      })
    }
  })
})

export default router
