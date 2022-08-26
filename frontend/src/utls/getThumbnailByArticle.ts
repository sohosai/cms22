import { AnyArticle } from '@/types/type'
import firebase from 'firebase'

export const getThumbnailByArticle = async (
  article: AnyArticle
): Promise<Blob> => {
  try {
    const articleId = article.id
    const userId = article.authorId
    const bodyRef = firebase
      .storage()
      .ref(`articleBodies/${userId}/${article.category}/${articleId}-thumbnail`)
    const url = await bodyRef.getDownloadURL()
    const response = await fetch(url)
    const file = await response.blob()
    return file
  } catch {
    return new Blob()
  }
}
