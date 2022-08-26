import { ContentCategory, ContentState } from '@/types/type'

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
    label: () => 'マイコンテンツ管理',
    path: () => '/contents',
  },
  postedContents: {
    label: () => 'コンテンツ審査',
    path: () => '/posted-contents',
  },
  downloadContents: {
    label: () => 'コンテンツダウンロード',
    path: () => '/download-contents',
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
  reflectAuthority: {
    label: () => '権限更新',
    path: () => '/reflect-authority',
  },
})

export const contentCategory: ContentCategory[] = [
  {
    value: 'unselected',
    label: '未選択',
  },
  {
    value: 'general-1',
    label: '一般企画',
    deadline: new Date(2021, 8, 31, 23, 59),
  },
  {
    value: 'stage-ondemand-1',
    label: 'オンラインステージ企画オンデマンド部門',
    deadline: new Date(2021, 8, 31, 23, 59),
  },
  {
    value: 'headquarters-1',
    label: '本部企画',
    deadline: new Date(2021, 10, 5, 23, 59),
  },
  {
    value: 'headquarters-auditor',
    label: '本部企画 (管理者のみ提出可)',
    deadline: new Date(3000, 1, 1, 0, 0),
  },
  {
    value: 'art-auditor',
    label: '芸祭企画 (管理者のみ提出可)',
    deadline: new Date(3000, 1, 1, 0, 0),
  },
  {
    value: 'stage-live-auditor',
    label: 'オンラインステージ (管理者のみ提出可)',
    deadline: new Date(3000, 1, 1, 0, 0),
  },
  {
    value: 'general-live-auditor',
    label: '一般ステージ (管理者のみ提出可)',
    deadline: new Date(3000, 1, 1, 0, 0),
  },
  {
    value: 'other-auditor',
    label: 'その他（管理者のみ提出可）',
    deadline: new Date(3000, 1, 1, 0, 0),
  },
]

export const contentState: ContentState[] = [
  {
    value: 'editable',
    label: '申請期間中',
  },
  {
    value: 'examined',
    label: '審査中',
  },
  {
    value: 'verified',
    label: '承認済み',
  },
  {
    value: 'rejection',
    label: '却下',
  },
]
