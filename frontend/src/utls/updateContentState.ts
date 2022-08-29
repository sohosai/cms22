import { ReviewState } from '@/types/type'
import firebase from 'firebase'

export const updateContentState = async (
  projectCode: string,
  state: ReviewState
): Promise<void> => {
  const url = `${process.env.VUE_APP_API_BASE}/contents/${projectCode}/review`
  const idToken = await firebase.auth().currentUser?.getIdToken(true)
  const authHeader = `Bearer ${idToken}`

  const data = {
    review_status: state,
  }

  await fetch(url, {
    method: 'PUT',
    headers: {
      Authorization: authHeader,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  })
}
