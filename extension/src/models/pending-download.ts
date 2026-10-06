import type { Download } from '../schemas/download.schema'

export type PendingDownload = Omit<Download, 'filename' | 'id'>
