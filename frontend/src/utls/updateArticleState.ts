import { ContentState } from '@/types/type'
import firebase from 'firebase'

export const updateArticleState = async (
  userId: string,
  articleId: string,
  state: ContentState
): Promise<void> => {
  await firebase
    .firestore()
    .doc(`contents/${userId}/articles/${articleId}`)
    .update({ state: state })
}
