import type { Download } from '../schemas/download.schema'

type PendingDownload = Omit<Download, 'filename' | 'id'>

export class DownloadTracker {
    constructor() {
        browser.downloads.onCreated.addListener((newDownload) => {
            console.log(
                `[DownloadTracker] Download created: [${newDownload.id}]: ${newDownload.filename}`
            )

            if (this._pendingDownload) {
                this.downloads.add({
                    id: newDownload.id,
                    filename: newDownload.filename,
                    tabId: this._pendingDownload.tabId,
                    minecraftVersion: this._pendingDownload.minecraftVersion,
                })

                this._pendingDownload = undefined
            }
        })

        browser.downloads.onChanged.addListener((changedDownload) => {
            if (changedDownload.state == undefined) return

            this.downloads.forEach((download) => {
                if (
                    download.id === changedDownload.id &&
                    changedDownload.state?.current === 'complete'
                ) {
                    this.downloads.delete(download)
                    this._onCompleted?.(download)
                }
            })
        })
    }

    private _onCompleted?: (download: Download) => void
    private _pendingDownload?: PendingDownload = undefined
    downloads: Set<Download> = new Set()

    onCompleted(callback: (download: Download) => void) {
        this._onCompleted = callback
    }

    start({ tabId, minecraftVersion }: PendingDownload) {
        this._pendingDownload = {
            tabId,
            minecraftVersion,
        }
    }
}
