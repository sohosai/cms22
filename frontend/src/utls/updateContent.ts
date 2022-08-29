import { Article } from '@/types/type'
import firebase from 'firebase'
import { updateThumbnail } from './updateThumbnail'

export const updateContent = async (
  content: Article,
  thumbnail: Blob
): Promise<void> => {
  const url = `${process.env.VUE_APP_API_BASE}/contents/${content.project_code}`
  const idToken = await firebase.auth().currentUser?.getIdToken(true)
  const authHeader = `Bearer ${idToken}`

  const data = {
    content_type: content.content_type,
    content_html: content.content_html,
    content_url: content.content_url,
  }

  await fetch(url, {
    method: 'PUT',
    headers: {
      Authorization: authHeader,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  })
  await updateThumbnail(content.project_code, thumbnail)
}
