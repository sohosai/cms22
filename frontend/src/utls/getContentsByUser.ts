import { ArticleOverview, FireStoreArticle } from '@/types/type'
import firebase from 'firebase'
import 'firebase/firestore'

/**
 * 投稿コンテンツ一覧を取得する
 */
export const getContentsByUserId = async (
  userId: string
): Promise<{ articles: ArticleOverview[] }> => {
  const articles: ArticleOverview[] = []
  await firebase
    .firestore()
    .collection(`contents/${userId}/articles`)
    .get()
    .then((querySnapshot) => {
      querySnapshot.forEach((doc) => {
        const article = doc.data() as FireStoreArticle
        articles.push({
          ...article,
          createAt: article.createAt.toDate(),
          updateAt: article.updateAt.toDate(),
        } as ArticleOverview)
      })
    })
  return { articles }
}
