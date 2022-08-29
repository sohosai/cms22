import { ContentState } from '@/types/type'

export const paths = Object.freeze({
  top: {
    label: () => 'トップ',
    path: () => '/',
  },
  signin: {
    label: () => 'サインイン',
    path: () => '/signin',
  },
  signout: {
    label: () => 'サインアウト',
    path: () => '/signout',
  },
  contents: {
    label: () => 'マイコンテンツ',
    path: () => '/contents',
  },
  postedContents: {
    label: () => 'コンテンツ審査',
    path: () => '/posted-contents',
  },
  contact: {
    label: () => 'お問い合わせ',
    path: () => '/contact',
  },
  faq: {
    label: () => 'よくある質問',
    path: () => '/faq',
  },
  editArticle: {
    label: (title: string) => title,
    path: (id: string) => `/contents/${id}/edit`,
  },
  auditArticle: {
    label: (title: string) => title,
    path: (id: string) => `/contents/${id}/audit`,
  },
})

export const contentState: ContentState[] = [
  {
    value: 'NeverSubmitted',
    label: '未提出',
  },
  {
    value: 'Pending',
    label: '審査待ち',
  },
  {
    value: 'Approved',
    label: '承認済み',
  },
  {
    value: 'Rejected',
    label: '却下',
  },
]
