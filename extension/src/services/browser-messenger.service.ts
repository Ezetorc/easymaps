export class BrowserMessenger {
    constructor() {
        browser.runtime.onMessage.addListener(
            (data: unknown, sender: browser.runtime.MessageSender) => {
                try {
                    this._onMessage?.(data, sender)
                } catch (error) {
                    console.error(`[BrowserMessenger] ${error}`)

                    if (sender.tab?.id != undefined) {
                        browser.tabs.sendMessage(sender.tab?.id, {
                            status: 'Error',
                        })
                    }
                }
            }
        )
    }

    private _onMessage?: (
        data: unknown,
        sender: browser.runtime.MessageSender
    ) => void

    onMessage(
        callback: (data: unknown, sender: browser.runtime.MessageSender) => void
    ) {
        this._onMessage = callback
    }
}
