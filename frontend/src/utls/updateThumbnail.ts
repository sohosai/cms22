import firebase from 'firebase'
import { convBlobToBase64 } from './convBlobToBase64'

export const updateThumbnail = async (
  projectCode: string,
  thumbnail: Blob
): Promise<void> => {
  const url = `${process.env.VUE_APP_API_BASE}/contents/${projectCode}/thumbnail`
  const idToken = await firebase.auth().currentUser?.getIdToken(true)
  const authHeader = `Bearer ${idToken}`

  const mime=thumbnail.type;
  const dataURL = await convBlobToBase64(thumbnail);
  console.log(dataURL)
  const regex=new RegExp(`data:${mime};base64,`);
  const base64 = dataURL.replace(regex, "");

  const data = {
    mime: mime,
    base64: thumbnail.size > 0 ? base64 : null,
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
