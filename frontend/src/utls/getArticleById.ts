import 'firebase/firestore'
import { Article, FireStoreAnyArticle, URLArticle } from '@/types/type'
import firebase from 'firebase'
import { completeArticle } from '@/utls/completeArticle'

export const getArticleById = async (
  id: string
): Promise<Article | URLArticle> => {
  const overview: FireStoreAnyArticle[] = []
  const querySnapshot = await firebase
    .firestore()
    .collectionGroup(`articles`)
    .where('id', '==', id)
    .get()
  querySnapshot.forEach((doc) => {
    overview.push(doc.data() as FireStoreAnyArticle)
  })
  const article = await completeArticle(overview[0])
  return article
}
