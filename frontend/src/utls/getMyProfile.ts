import { UserProfile } from '@/types/type'
import firebase from 'firebase'

export const getMyProfile = async (): Promise<UserProfile> => {
  const url = `${process.env.VUE_APP_API_BASE}/me`
  const idToken = await firebase.auth().currentUser?.getIdToken(true)
  const authHeader = `Bearer ${idToken}`
  const response = await fetch(url, { headers: { Authorization: authHeader } })
  const body = await response.json()
  return body as UserProfile
}
