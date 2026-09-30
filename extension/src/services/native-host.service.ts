export class NativeHost {
    constructor(applicationName: string) {
        const port = browser.runtime.connectNative(applicationName)

        this._port = port

        port.onMessage.addListener((message) => {
            console.log('[NativeHost] Message received: ', message)

            this._onMessage?.(message)
        })

        port.onDisconnect.addListener(() => {
            console.warn('[NativeHost] Port disconnected')
        })
    }

    private _onMessage?: (message: unknown) => void = undefined
    private _port: browser.runtime.Port

    send(message: object) {
        this._port.postMessage(message)
    }

    onMessage(callback: (message: unknown) => void) {
        this._onMessage = callback
    }
}
