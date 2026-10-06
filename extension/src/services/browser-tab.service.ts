import { parse } from 'valibot'
import type { AppResponse } from '../schemas/app-response.schema'
import {
    AdapterRequestSchema,
    type AdapterRequest,
} from '../schemas/adapter-request.schema'

export class BrowserTab {
    constructor(public readonly id: number) {}

    private _onRequest?: (request: AdapterRequest) => void

    receive(message: unknown) {
        try {
            const request = parse(AdapterRequestSchema, message)

            console.log(`[BrowserTab: ${this.id}] Request received:`, request)

            this._onRequest?.(request)
        } catch (error) {
            console.error(
                `[BrowserTab: ${this.id}] Error parsing request:`,
                error
            )
        }
    }

    onRequest(callback: (request: AdapterRequest) => void) {
        this._onRequest = callback
    }

    send(response: AppResponse) {
        browser.tabs.sendMessage(this.id, response)
    }
}
