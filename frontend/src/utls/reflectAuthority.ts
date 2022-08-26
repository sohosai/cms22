import firebase from 'firebase'
import 'firebase/functions'

export const reflectAuthority = async (): Promise<{
  ok: boolean
  message: string
}> => {
  try {
    const res = await firebase.functions().httpsCallable('reflectAuthority')()
    if (res.data.status === 200) {
      return { ok: true, message: '' }
    } else {
      return { ok: false, message: res.data.message as string }
    }
  } catch {
    return { ok: false, message: 'エラーが発生しました。' }
  }
}
