import { parse } from 'valibot'
import {
    AppResponseSchema,
    type AppResponse,
} from '../schemas/app-response.schema'
import type { AppRequest } from '../schemas/app-request.schema'

export class NativeHost {
    private readonly _port: browser.runtime.Port
    private _onResponse?: (response: AppResponse) => void

    constructor(applicationName: string) {
        this._port = browser.runtime.connectNative(applicationName)

        this._port.onDisconnect.addListener(() => {
            console.warn(`[NativeHost: ${applicationName}] Port disconnected`)
        })

        this._port.onMessage.addListener((message) => {
            console.log('[NativeHost] Response received:', message)

            try {
                const response = parse(AppResponseSchema, message)

                this._onResponse?.(response)
            } catch (error) {
                console.error(`[NativeHost] Error parsing response:`, error)
            }
        })
    }

    send(request: AppRequest) {
        this._port.postMessage(request)
    }

    onResponse(callback: (response: AppResponse) => void) {
        this._onResponse = callback
    }
}
