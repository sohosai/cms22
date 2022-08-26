export const convBlobToBase64 = (file: Blob): Promise<string> =>
  new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.readAsDataURL(file)
    reader.onload = () => {
      if (!reader.result) return
      resolve(reader.result as string)
    }

    reader.onerror = (error) => reject(error)
  })
