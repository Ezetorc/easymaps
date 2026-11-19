import { Button } from '../components/button.component'
import { ERROR_MESSAGES } from '../constants/error-messages.constants'
import { WebAdapter } from '../models/web-adapter.model'
import type { AppResponse } from '../schemas/app-response.schema'
import { Background } from '../services/background.service'

class MinecraftMapsAdapter extends WebAdapter {
    getMinecraftVersion(): string | undefined {
        const $minecraftVersion = document.querySelector(
            'div[class="grid grid-cols-2 overflow-hidden rounded-[4px] border border-white/[0.09] bg-white/[0.025]"]'
        )
        const minecraftVersion =
            $minecraftVersion?.childNodes[1]?.childNodes[1]?.textContent

        return minecraftVersion ?? undefined
    }

    isValidPage(): boolean {
        const { pathname } = window.location

        return /^\/[^/]+\/?$/.test(pathname)
    }

    isValidMap(): boolean {
        const isJavaEdition = !document.querySelector('a[href="/bedrock"]')

        if (!isJavaEdition) {
            this.$button.setWarning("EasyMaps doesn't support Bedrock yet...")
            return false
        }

        const isModded = document.querySelector('a[href="/modded"]')

        if (isModded) {
            this.$button.setWarning(
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
            this.$button.setWarning(
                "EasyMaps doesn't work with external downloads yet..."
            )
            return false
        }

        return true
    }

    addResponseEvent(minecraftMapName: string) {
        browser.runtime.onMessage.addListener((response: AppResponse) => {
            if (response.status === 'MapCheckResult') {
                this.$button.element.remove()

                if (response.world_exists) {
                    this.$button = new Button()

                    this.$button.addAction('Change username', () => {
                        this.minecraftUsername =
                            prompt('Enter a new username', 'EasyMaps') ??
                            undefined
                    })
                    this.$button.addAction('Delete map', () => {
                        this.$button.setLoading('Deleting map instance...')

                        Background.send({
                            action: 'DeleteMap',
                            worldName: minecraftMapName,
                        })
                    })
                    this.$button.setIdle('Continue playing')
                    this.$button.onClick = () => {
                        const minecraftVersion = this.getMinecraftVersion()

                        Background.send({
                            action: 'ContinueMap',
                            worldName: minecraftMapName,
                            minecraftVersion,
                            minecraftUsername: this.minecraftUsername,
                        })
                    }

                    this.addButton()
                } else {
                    this.$button = new Button()

                    this.$button.addAction('Change username', () => {
                        this.minecraftUsername =
                            prompt('Enter a new username', 'EasyMaps') ??
                            undefined
                    })

                    this.$button.setIdle('Play map')
                    this.$button.onClick = () => {
                        const clickEvent = new Event('click', { bubbles: true })
                        const $downloadButton =
                            document.querySelector('.mc-download-btn')

                        if ($downloadButton) {
                            $downloadButton.dispatchEvent(clickEvent)
                            this.$button.setLoading('Downloading map...')

                            const minecraftVersion = this.getMinecraftVersion()

                            Background.send({
                                action: 'StartMap',
                                minecraftVersion,
                                minecraftUsername: this.minecraftUsername,
                            })
                        }
                    }

                    this.addButton()
                }
            } else if (response.status === 'MapDeleteSuccess') {
                window.location.reload()
            } else if (response.status === 'ImportingMap') {
                this.$button.setLoading('Importing world...')
            } else if (response.status === 'InstallingVersion') {
                this.$button.setLoading('Installing Minecraft version...')
            } else if (response.status === 'LaunchingMinecraft') {
                this.$button.setLoading('Launching Minecraft...')

                setTimeout(
                    () => this.$button.setSuccess('Enjoy the adventure!'),
                    8000
                )
            } else if (response.status === 'AppError') {
                const message = ERROR_MESSAGES[response.error]

                this.$button.setError(message)
            }
        })
    }

    addButton() {
        const $downloadButton = document.querySelector('.mc-download-btn')

        if (!$downloadButton) {
            return
        }

        $downloadButton.before(this.$button.element)
    }

    getMapName() {
        return document.querySelector('h1')?.textContent
    }

    checkMap(mapName: string) {
        Background.send({
            action: 'CheckMap',
            worldName: mapName,
        })

        this.$button.setLoading('Checking map...')
    }
}

new MinecraftMapsAdapter().run()
