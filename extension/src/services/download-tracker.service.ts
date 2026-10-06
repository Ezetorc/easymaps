import type { PendingDownload } from '../models/pending-download'
import type { Download } from '../schemas/download.schema'

export class DownloadTracker {
    private static _eventsSetup = false
    private static _onMapDownloadCompleted?: (download: Download) => void
    private static _pendingDownload?: PendingDownload
    public static downloads: Set<Download> = new Set()

    public static setupEvents(): void {
        if (DownloadTracker._eventsSetup) return

        DownloadTracker._eventsSetup = true

        browser.downloads.onCreated.addListener((newDownload) => {
            console.log(
                `[DownloadTracker] Download created: [${newDownload.id}]: ${newDownload.filename}`
            )

            if (DownloadTracker._pendingDownload) {
                DownloadTracker.downloads.add({
                    id: newDownload.id,
                    filename: newDownload.filename,
                    tabId: DownloadTracker._pendingDownload.tabId,
                    minecraftVersion:
                        DownloadTracker._pendingDownload.minecraftVersion,
                })

                DownloadTracker._pendingDownload = undefined
            }
        })

        browser.downloads.onChanged.addListener((changedDownload) => {
            if (changedDownload.state == undefined) return

            DownloadTracker.downloads.forEach((download) => {
                if (
                    download.id === changedDownload.id &&
                    changedDownload.state?.current === 'complete'
                ) {
                    DownloadTracker.downloads.delete(download)
                    DownloadTracker._onMapDownloadCompleted?.(download)
                }
            })
        })

        console.log('[DownloadTracker] Events successfully setup')
    }

    public static onMapDownloadCompleted(
        callback: (download: Download) => void
    ): void {
        DownloadTracker._onMapDownloadCompleted = callback
    }

    public static start({ tabId, minecraftVersion }: PendingDownload): void {
        DownloadTracker._pendingDownload = {
            tabId,
            minecraftVersion,
        }
    }
}
