export type ReviewState = 'NeverSubmitted' | 'Pending' | 'Approved' | 'Rejected'

export type ContentType = 'ArticleContent' | 'LinkContent'

export type ContentState = {
  value: ReviewState
  label: string
}

export type Article = {
  content_type: ContentType
  project_code: string
  thumbnail: string | null
  content_html: string
  content_url: string
  status: ReviewState
  owner: UserProfile
  subwoner: UserProfile
  project_name: string
  project_category: string
  is_academic: boolean
  is_art: boolean
  crated_at: Date
  updated_at: Date
}

export type UserProfile = {
  name: string
  email: string
  is_committee: boolean
}

export type SaveAbility = {
  is_my_project: boolean
  in_save_period: boolean
  is_editable: boolean
  is_committee: boolean
  total: boolean
}
