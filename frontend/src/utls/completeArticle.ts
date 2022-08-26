import {
  Article,
  FireStoreAnyArticle,
  URLArticle,
  isFireStoreURLArticle,
} from '@/types/type'
import firebase from 'firebase'

export const completeArticle = async (
  overview: FireStoreAnyArticle
): Promise<Article | URLArticle> => {
  if (isFireStoreURLArticle(overview)) {
    // URL Article
    const article: URLArticle = {
      ...overview,
      createAt: overview.createAt.toDate(),
      updateAt: overview.updateAt.toDate(),
    }
    return article
  } else {
    // HTML Article
    const article: Article = {
      ...overview,
      contentHtml: '',
      createAt: overview.createAt.toDate(),
      updateAt: overview.updateAt.toDate(),
    }
    const userId = article.authorId
    const articleId = article.id
    if (overview.category !== 'unselected') {
      const bodyRef = firebase
        .storage()
        .ref(`articleBodies/${userId}/${overview.category}/${articleId}`)
      const url = await bodyRef.getDownloadURL()
      const response = await fetch(url)
      const text = await response.text()
      article.contentHtml = text
    }
    return article
  }
}
