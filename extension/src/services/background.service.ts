import type { AdapterRequest } from '../schemas/adapter-request.schema'

export class Background {
    static send(request: AdapterRequest) {
        browser.runtime.sendMessage(request)
    }

    static onResponse(callback: (response: Response) => void) {
        browser.runtime.onMessage.addListener(callback)
    }
}
