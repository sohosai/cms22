import firebase from 'firebase'
import 'firebase/auth'

export const getUser = (): firebase.User | null => {
  return firebase.auth().currentUser
}
