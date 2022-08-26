import { Article, URLArticle, AnyArticle, isURLArticle } from '@/types/type'
import firebase from 'firebase'
import { updateThumbnailByUserAndId } from './updateThumbnailByUserAndId'

export const updateArticleByUserAndId = async (
  anyArticle: AnyArticle,
  thumbnail?: Blob
): Promise<void> => {
  const userId = anyArticle.authorId
  console.log(anyArticle)
  if (isURLArticle(anyArticle)) {
    const article = anyArticle as URLArticle
    const doc = {
      ...article,
      createAt: firebase.firestore.Timestamp.fromDate(article.createAt),
      updateAt: firebase.firestore.Timestamp.now(),
    }

    const articleId = article.id
    await firebase
      .firestore()
      .doc(`contents/${userId}/articles/${articleId}`)
      .update(doc)
  } else {
    const article = anyArticle as Article
    const _doc = {
      ...article,
      createAt: firebase.firestore.Timestamp.fromDate(article.createAt),
      updateAt: firebase.firestore.Timestamp.now(),
    }
    const articleId = article.id
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const { contentHtml, ...doc } = _doc
    await firebase
      .firestore()
      .doc(`contents/${userId}/articles/${articleId}`)
      .update(doc)
    const bodyRef = firebase
      .storage()
      .ref(`articleBodies/${userId}/${article.category}/${articleId}`)
    await bodyRef.putString(article.contentHtml)
  }
  if (anyArticle.category.startsWith('headquarters') && thumbnail) {
    await updateThumbnailByUserAndId(anyArticle, thumbnail)
  }
}
