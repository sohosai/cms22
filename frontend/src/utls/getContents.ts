import { Article } from '@/types/type'
import firebase from 'firebase'

/**
 * コンテンツ一覧を取得する
 * auditor 以外が使用すると権限エラーになる
 */
export const getContents = async (): Promise<Article[]> => {
  const url = `${process.env.VUE_APP_API_BASE}/contents/list`
  const idToken = await firebase.auth().currentUser?.getIdToken(true)
  const authHeader = `Bearer ${idToken}`
  const response = await fetch(url, { headers: { Authorization: authHeader } })
  const body = await response.json()
  return body.contents
}
