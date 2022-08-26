import _sanitizeHtml from 'sanitize-html'

export const sanitizeHtml = (html: string): string => {
  const sanitizedHtml = _sanitizeHtml(html, {
    allowedTags: _sanitizeHtml.defaults.allowedTags.concat(['img', 'iframe']),
    allowedAttributes: {
      a: ['href', 'name', 'target'],
      h1: ['style', 'class'],
      h2: ['style', 'class'],
      iframe: ['src', 'allowfullscreen'],
      img: ['src'],
      p: ['style', 'class'],
      span: ['style', 'class'],
    },
    allowedSchemesByTag: {
      img: ['data'],
    },
    allowedIframeHostnames: ['www.youtube.com'],
  })
  return sanitizedHtml
}
