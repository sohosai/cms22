import { AnyArticle, FireStoreAnyArticle } from '@/types/type'
import firebase from 'firebase'
import { completeArticle } from '@/utls/completeArticle'
import { getArticleById } from './getArticleById'

export const getArticleByUserAndId = async (
  userId: string,
  articleId: string
): Promise<AnyArticle> => {
  const overviewDoc = await firebase
    .firestore()
    .doc(`contents/${userId}/articles/${articleId}`)
    .get()
  if (overviewDoc.exists) {
    const overview = overviewDoc.data() as FireStoreAnyArticle
    const article = await completeArticle(overview)
    return article
  } else {
    const article = await getArticleById(articleId)
    return article
  }
}
