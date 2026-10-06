import { ERROR_MESSAGES } from '../constants/error-messages.constants'
import { WebAdapter } from '../models/web-adapter.model'
import { type AppResponse } from '../schemas/app-response.schema'
import '../models/play-button.model'
import { Background } from '../services/background.service'

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
                this.playButton.setDownloading()

                Background.send({
                    action: 'Start',
                    minecraftVersion,
                })
            }
        })
    }

    addResponseEvent() {
        browser.runtime.onMessage.addListener((response: AppResponse) => {
            if (response.status === 'Importing') {
                this.playButton.setImporting()
            } else if (response.status === 'Installing') {
                this.playButton.setInstalling()
            } else if (response.status === 'Launching') {
                this.playButton.setLaunching()

                setTimeout(() => this.playButton.setFinished(), 8000)
            } else if (response.status === 'AppError') {
                const message = ERROR_MESSAGES[response.error]

                this.playButton.setError(message)
            }
        })
    }
}

new MinecraftMapsAdapter().run()
