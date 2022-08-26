import firebase from 'firebase'

export type ContentCategoryValue =
  | 'unselected'
  | 'general-1'
  | 'general-2'
  | 'general-3'
  | 'general-4'
  | 'general-auditor'
  | 'stage-ondemand-1'
  | 'stage-ondemand-2'
  | 'stage-ondemand-3'
  | 'stage-ondemand-auditor'
  | 'headquarters-1'
  | 'headquarters-auditor'
  | 'academic'
  | 'art-auditor'
  | 'stage-live-auditor'
  | 'general-live-auditor'
  | 'other-auditor'

export type ContentStateValue =
  | 'editable'
  | 'examined'
  | 'verified'
  | 'rejection'

export type ContentCategory = {
  value: ContentCategoryValue
  label: string
  deadline?: Date
}

export type ContentState = {
  value: ContentStateValue
  label: string
}

export type ArticleTypeValue = 'html' | 'url'

type CommonArticle = {
  id: string
  title: string
  category: ContentCategoryValue
  academic: boolean
  authorId: string
  state: ContentStateValue
  articleType: ArticleTypeValue
  projectName: string
}

export type ArticleOverview = CommonArticle & {
  createAt: Date
  updateAt: Date
}

export type Article = ArticleOverview & {
  contentHtml: string
}

export type URLArticle = ArticleOverview & {
  url: string
}

export type AnyArticle = Article | URLArticle

export function isURLArticle(arg: AnyArticle): arg is URLArticle {
  return arg.articleType === 'url'
}

export type FireStoreArticle = CommonArticle & {
  createAt: firebase.firestore.Timestamp
  updateAt: firebase.firestore.Timestamp
}

export type FireStoreURLArticle = FireStoreArticle & {
  url: string
}

export type FireStoreAnyArticle = FireStoreArticle | FireStoreURLArticle

export function isFireStoreURLArticle(
  arg: FireStoreAnyArticle
): arg is FireStoreURLArticle {
  return arg.articleType === 'url'
}

export type FunctionsResponse = {
  status: number
  message: string
}

export type UserProfile = {
  name: string
  email: string
  verified: boolean
}
