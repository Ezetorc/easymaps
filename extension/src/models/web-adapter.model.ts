import { Button } from '../components/button.component'

export abstract class WebAdapter {
    protected $button: Button = new Button()
    protected minecraftUsername?: string

    abstract isValidPage(): boolean
    abstract isValidMap(): boolean
    abstract addButton(): void
    abstract addResponseEvent(mapName: string): void
    abstract getMapName(): string | undefined
    abstract checkMap(mapName: string): void

    run() {
        if (!this.isValidPage()) return

        this.addButton()

        const mapName = this.getMapName()

        if (!mapName) {
            this.$button.setError("Couldn't determine map's name")
            return
        }

        if (!this.isValidMap()) return

        this.checkMap(mapName)
        this.addResponseEvent(mapName)
    }
}
