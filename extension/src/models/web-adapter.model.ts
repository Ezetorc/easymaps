import { PlayButton } from './play-button.model'

export abstract class WebAdapter {
    protected playButton: PlayButton = new PlayButton()

    run() {
        if (!this.isValidPage()) return

        this.addPlayButton()

        if (!this.isValidMap()) return

        // this.addDownloadEvent(minecraftVersion)
        this.addResponseEvent()
    }

    abstract isValidPage(): boolean
    abstract isValidMap(): boolean
    abstract addPlayButton(): void
    // abstract addDownloadEvent(minecraftVersion?: string): void
    abstract addResponseEvent(): void
}
