class AbstractWebAdapter {
    constructor() {
        if (this.constructor === AbstractWebAdapter) {
            throw Error("Can't instanciate an abstract class")
        }
    }

    playButton = null

    run() {
        if (!this.isValidPage()) return

        this.createPlayButton()
        this.addPlayButton()

        if (!this.isValidMap()) return

        const minecraftVersion = this.determineMinecraftVersion()

        this.addDownloadEvent(minecraftVersion)
        this.addResponseEvent()
    }

    isValidPage() {
        throw Error("Method 'isValidPage' must be implemented in subclass")
    }

    createPlayButton() {
        throw Error("Method 'createPlayButton' must be implemented in subclass")
    }

    isValidMap() {
        throw Error("Method 'isValidMap' must be implemented in subclass")
    }

    addPlayButton() {
        throw Error("Method 'addPlayButton' must be implemented in subclass")
    }

    determineMinecraftVersion() {
        throw Error("Method 'determineMinecraftVersion' must be implemented in subclass")
    }

    addDownloadEvent(minecraftVersion) {
        throw Error("Method 'addDownloadEvent' must be implemented in subclass")
    }

    addResponseEvent() {
        throw Error("Method 'addResponseEvent' must be implemented in subclass")
    }
}