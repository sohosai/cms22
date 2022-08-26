import { ArticleOverview, FireStoreArticle } from '@/types/type'
import firebase from 'firebase'
import 'firebase/firestore'

/**
 * コンテンツ一覧を取得する
 * auditor 以外が使用すると権限エラーになる
 */
export const getContents = async (): Promise<{
  articles: ArticleOverview[]
}> => {
  const articles: ArticleOverview[] = []
  await firebase
    .firestore()
    .collectionGroup(`articles`)
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
