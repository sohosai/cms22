import { SaveAbility } from '@/types/type'
import firebase from 'firebase'

export const checkSaveAbility = async (
  projectCode: string
): Promise<SaveAbility> => {
  const url = `${process.env.VUE_APP_API_BASE}/contents/${projectCode}/save-ability`
  const idToken = await firebase.auth().currentUser?.getIdToken(true)
  const authHeader = `Bearer ${idToken}`
  const response = await fetch(url, { headers: { Authorization: authHeader } })
  const body = await response.json()
  return body as SaveAbility
}
