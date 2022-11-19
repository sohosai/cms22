import { ContentState } from '@/types/type'

export const paths = Object.freeze({
  top: {
    label: () => 'トップ',
    path: () => '/',
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
