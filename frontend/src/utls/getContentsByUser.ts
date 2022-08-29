import { Article } from '@/types/type'
import firebase from 'firebase'
import 'firebase/firestore'

/**
 * 投稿コンテンツ一覧を取得する
 */
export const getContentsByUser = async (): Promise<Article[]> => {
  const url = `${process.env.VUE_APP_API_BASE}/contents/my`
  const idToken = await firebase.auth().currentUser?.getIdToken(true)
  const authHeader = `Bearer ${idToken}`
  const response = await fetch(url, { headers: { Authorization: authHeader } })
  const body = await response.json()
  return body.contents
}
