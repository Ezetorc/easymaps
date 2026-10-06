import { NativeHost } from './services/native-host.service'
import { BrowserTab } from './services/browser-tab.service'
import { DownloadTracker } from './services/download-tracker.service'
import type { AdapterRequest } from './schemas/adapter-request.schema'

console.log('[Background] Loaded')

const nativeHost = new NativeHost('easymaps')
const tabs = new Map<number, BrowserTab>()

DownloadTracker.setupEvents()

function getBrowserTab(tabId: number) {
    let tab = tabs.get(tabId)

    if (!tab) {
        tab = new BrowserTab(tabId)

        tab.onRequest((request: AdapterRequest) => {
            if (request.action === 'Start') {
                DownloadTracker.start({
                    tabId: tabId,
                    minecraftVersion: request.minecraftVersion,
                })
            }
        })

        tabs.set(tabId, tab)
    }

    return tab
}

browser.runtime.onMessage.addListener((message, sender) => {
    const tabId = sender.tab?.id

    if (tabId === undefined) return

    const tab = getBrowserTab(tabId)

    tab.receive(message)
})

DownloadTracker.onMapDownloadCompleted((download) => {
    nativeHost.send({
        action: 'Start',
        minecraft_version: download.minecraftVersion,
        download_path: download.filename,
        requester_id: download.tabId,
    })
})

nativeHost.onResponse((response) => {
    if (response.status === 'ConnectionError') {
        alert('[EasyMaps WebExtension] Connection error with native host app')
        return
    }

    const tab = tabs.get(response.requester_id)

    if (!tab) {
        console.warn(
            `[Background] Could not find BrowserTab ${response.requester_id}`
        )
        return
    }

    tab.send(response)
})
