import { Article } from '@/types/type'
import firebase from 'firebase'

export const getContentByPj = async (projectCode: string): Promise<Article> => {
  const url = `${process.env.VUE_APP_API_BASE}/contents/${projectCode}`
  const idToken = await firebase.auth().currentUser?.getIdToken(true)
  const authHeader = `Bearer ${idToken}`
  const response = await fetch(url, { headers: { Authorization: authHeader } })
  const body = await response.json()
  return body.content as Article
}
