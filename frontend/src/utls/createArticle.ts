import { FireStoreArticle, FireStoreURLArticle } from '@/types/type'
import firebase from 'firebase'

export const createArticle = async (userId: string): Promise<string> => {
  const now = firebase.firestore.Timestamp.now()
  const doc: FireStoreArticle = {
    id: '',
    articleType: 'html',
    projectName: '',
    title: now.toDate().toLocaleString() + ' に作成した記事',
    updateAt: now,
    createAt: now,
    authorId: userId,
    state: 'editable',
    category: 'unselected',
    academic: false,
  }
  const ref = await firebase
    .firestore()
    .collection(`contents/${userId}/articles`)
    .doc()
  const articleId = ref.id
  ref.set({ ...doc, id: ref.id })
  return articleId
}

export const createURLArticle = async (userId: string): Promise<string> => {
  const now = firebase.firestore.Timestamp.now()
  const doc: FireStoreURLArticle = {
    id: '',
    articleType: 'url',
    projectName: '',
    title: now.toDate().toLocaleString() + ' に投稿したURL',
    updateAt: now,
    createAt: now,
    authorId: userId,
    state: 'editable',
    category: 'unselected',
    academic: false,
    url: '',
  }
  const ref = await firebase
    .firestore()
    .collection(`contents/${userId}/articles`)
    .doc()
  const articleId = ref.id
  ref.set({ ...doc, id: ref.id })
  return articleId
}
