import '../models/play-button.model'
import { safeParse } from 'valibot'

import { WebAdapter } from '../models/web-adapter.model'
import { ResponseSchema, ResponseStatus } from '../schemas/response.schema'

class MinecraftMapsAdapter extends WebAdapter {
    isValidPage(): boolean {
        const { pathname } = window.location

        return /^\/[^/]+\/?$/.test(pathname)
    }

    isValidMap(): boolean {
        const isJavaEdition = !document.querySelector('a[href="/bedrock"]')

        if (!isJavaEdition) {
            this.playButton.setUnsupported(
                "EasyMaps doesn't support Bedrock yet..."
            )
            return false
        }

        const isModded = document.querySelector('a[href="/modded"]')

        if (isModded) {
            this.playButton.setUnsupported(
                "EasyMaps doesn't support modded maps yet..."
            )
            return false
        }

        const isExternal = document.evaluate(
            '//p[contains(., "Opens in new tab")]',
            document,
            null,
            XPathResult.FIRST_ORDERED_NODE_TYPE,
            null
        ).singleNodeValue

        if (isExternal) {
            this.playButton.setUnsupported(
                "EasyMaps doesn't work with external downloads yet..."
            )
            return false
        }

        return true
    }

    addPlayButton() {
        const $downloadButton = document.querySelector('.mc-download-btn')

        if ($downloadButton) {
            $downloadButton.before(this.playButton.element)
        }
    }

    determineMinecraftVersion(): string | undefined {
        const $minecraftVersion = document.querySelector(
            'div[class="grid grid-cols-2 overflow-hidden rounded-[4px] border border-white/[0.09] bg-white/[0.025]"]'
        )
        const minecraftVersion =
            $minecraftVersion?.childNodes[1]?.childNodes[1]?.textContent

        return minecraftVersion ?? undefined
    }

    addDownloadEvent(minecraftVersion?: string) {
        this.playButton.onClick(() => {
            const clickEvent = new Event('click', { bubbles: true })
            const $downloadButton = document.querySelector('.mc-download-btn')

            if ($downloadButton) {
                $downloadButton.dispatchEvent(clickEvent)

                browser.runtime.sendMessage({
                    action: 'Start',
                    minecraftVersion,
                })
            }
        })
    }

    addResponseEvent() {
        browser.runtime.onMessage.addListener((message: unknown) => {
            const result = safeParse(ResponseSchema, message)

            if (!result.success) return

            const { output: response } = result

            if (response.status === ResponseStatus.Starting) {
                this.playButton.setStarting()
            } else if (response.status === ResponseStatus.Finished) {
                this.playButton.setFinished()
            } else if (response.status === ResponseStatus.AppError) {
                this.playButton.setError()
            }
        })
    }
}

new MinecraftMapsAdapter().run()
