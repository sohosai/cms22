import { Article, isURLArticle } from '@/types/type'
import { getArticleByUserAndId } from './getArticleByUserAndId'
import firebase from 'firebase'

export const deleteArticle = async (
  articleId: string,
  userId: string
): Promise<void> => {
  const anyArticle = await getArticleByUserAndId(userId, articleId)
  await firebase
    .firestore()
    .doc(`contents/${userId}/articles/${articleId}`)
    .delete()
  if (!isURLArticle(anyArticle)) {
    const article = anyArticle as Article
    try {
      await firebase
        .storage()
        .ref(`articleBodies/${userId}/${article.category}/${articleId}`)
        .delete()
    } catch (e) {
      // TODO
    }
  }
}
