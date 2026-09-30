import { safeParse } from 'valibot'
import { NativeHost } from './services/native-host.service'
import { DownloadTracker } from './services/download-tracker.service'
import { BrowserMessenger } from './services/browser-messenger.service'
import { ResponseSchema, ResponseStatus } from './schemas/response.schema'
import { RequestAction, RequestSchema } from './schemas/request.schema'

console.log('[Background] Loaded')

const nativeHost = new NativeHost('easymaps')
const downloadTracker = new DownloadTracker()
const browserMessenger = new BrowserMessenger()

nativeHost.onMessage((message) => {
    const result = safeParse(ResponseSchema, message)

    if (result.success) {
        const { output: response } = result

        if (response.status == ResponseStatus.ConnectionError) {
            alert(
                'Connection error with native host app. Please report this bug'
            )
        } else if (response.webTabId != undefined) {
            browser.tabs.sendMessage(response.webTabId, response)
        }
    }
})

downloadTracker.onCompleted((download) => {
    nativeHost.send({
        action: 'Start',
        minecraft_version: download.minecraftVersion,
        download_path: download.filename,
        web_tab_id: download.tabId,
    })
})

browserMessenger.onMessage((data, sender) => {
    const result = safeParse(RequestSchema, data)

    if (result.success) {
        const { output: request } = result

        if (request.action == RequestAction.Start) {
            downloadTracker.start({
                tabId: sender.tab?.id,
                minecraftVersion: request.minecraftVersion,
            })
        }
    }
})
