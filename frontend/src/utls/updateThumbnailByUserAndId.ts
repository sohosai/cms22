import { AnyArticle } from '@/types/type'
import firebase from 'firebase'

export const updateThumbnailByUserAndId = async (
  article: AnyArticle,
  thumbnail: Blob
): Promise<void> => {
  const articleId = article.id
  const bodyRef = firebase
    .storage()
    .ref(
      `articleBodies/${article.authorId}/${article.category}/${articleId}-thumbnail`
    )
  await bodyRef.put(thumbnail)
}
