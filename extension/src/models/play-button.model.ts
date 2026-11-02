export class PlayButton {
    private readonly button: HTMLButtonElement
    private onPlay?: () => void

    constructor() {
        this.button = document.createElement('button')

        this.setupStyles()
        this.setupEvents()
    }

    get element(): HTMLButtonElement {
        return this.button
    }

    onClick(callback: () => void): void {
        this.onPlay = callback
    }

    setChecking(): void {
        this.button.textContent = 'Checking map information...'
        this.button.disabled = true
        this.button.style.cursor = 'not-allowed'
        this.button.style.backgroundColor = '#ed4337'
    }

    setStart(): void {
        this.button.textContent = 'Play with EasyMaps'
        this.button.disabled = false
        this.button.style.cursor = 'pointer'
        this.button.style.backgroundColor = '#24aae2'
    }

    setContinue(): void {
        this.button.textContent = 'Continue playing'
        this.button.disabled = false
        this.button.style.cursor = 'pointer'
        this.button.style.backgroundColor = '#24aae2'
    }

    setDownloading() {
        this.button.textContent = 'Downloading map...'
        this.button.disabled = true
        this.button.style.cursor = 'wait'
        this.button.style.backgroundColor = '#24aae2'
    }

    setImporting() {
        this.button.textContent = 'Importing Minecraft world...'
        this.button.disabled = true
        this.button.style.cursor = 'wait'
        this.button.style.backgroundColor = '#24aae2'
    }

    setInstalling() {
        this.button.textContent = 'Installing Minecraft version...'
        this.button.disabled = true
        this.button.style.cursor = 'wait'
        this.button.style.backgroundColor = '#24aae2'
    }

    setLaunching() {
        this.button.textContent = 'Launching Minecraft...'
        this.button.disabled = true
        this.button.style.cursor = 'wait'
        this.button.style.backgroundColor = '#24aae2'
    }

    setFinished(): void {
        this.button.textContent = 'Enjoy the adventure!'
        this.button.disabled = true
        this.button.style.cursor = 'not-allowed'
        this.button.style.backgroundColor = '#24aae2'
    }

    setUnsupported(message: string): void {
        this.button.textContent = message
        this.button.disabled = true
        this.button.style.backgroundColor = '#ed4337'
        this.button.style.cursor = 'not-allowed'
    }

    setError(message?: string): void {
        this.button.textContent = message ?? 'An error occurred! Try again'
        this.button.disabled = true
        this.button.style.backgroundColor = '#ed4337'
        this.button.style.cursor = 'not-allowed'
    }

    private setupStyles(): void {
        this.button.style.width = '100%'
        this.button.style.padding = '1rem 1.5rem'
        this.button.style.marginBottom = '16px'
        this.button.style.backgroundColor = '#24aae2'
        this.button.style.borderRadius = '4px'
        this.button.style.color = 'black'
        this.button.style.fontWeight = 'bold'
        this.button.style.transition =
            'transform 0.2s ease, box-shadow 0.2s ease, background-color 0.2s ease'
    }

    private setupEvents(): void {
        this.button.addEventListener('click', () => {
            this.onPlay?.()
        })

        this.button.addEventListener('mouseenter', () => {
            if (this.button.disabled) return

            this.button.style.transform = 'translateY(-3px)'
            this.button.style.boxShadow = '0 6px 18px rgba(36, 170, 226, 0.4)'
            this.button.style.backgroundColor = '#35b8ed'
        })

        this.button.addEventListener('mouseleave', () => {
            if (this.button.disabled) return

            this.button.style.transform = 'translateY(0)'
            this.button.style.boxShadow = '0 2px 6px rgba(36, 170, 226, 0.4)'
            this.button.style.backgroundColor = '#24aae2'
        })

        this.button.addEventListener('mousedown', () => {
            if (this.button.disabled) return

            this.button.style.transform = 'translateY(-1px) scale(0.98)'
        })

        this.button.addEventListener('mouseup', () => {
            if (this.button.disabled) return

            this.button.style.transform = 'translateY(-3px)'
        })
    }
}
