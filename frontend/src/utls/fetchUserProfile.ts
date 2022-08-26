import { UserProfile } from '@/types/type'
import firebase from 'firebase'
import 'firebase/functions'

export const fetchUserProfile = async (
  userId: string
): Promise<{
  ok: boolean
  message: string
  profile?: UserProfile
}> => {
  try {
    const res = await firebase.functions().httpsCallable('getUserProfile')(
      userId
    )
    if (res.data.status === 200) {
      return { ok: true, message: '', profile: res.data.profile }
    } else {
      return { ok: false, message: res.data.message as string }
    }
  } catch {
    return { ok: false, message: 'エラーが発生しました。' }
  }
}
