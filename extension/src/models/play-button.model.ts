export class PlayButton {
    private readonly button: HTMLButtonElement
    private onPlay?: () => void

    constructor() {
        this.button = document.createElement('button')

        this.setupStyles()
        this.setupEvents()
        this.setReady()
    }

    get element(): HTMLButtonElement {
        return this.button
    }

    onClick(callback: () => void): void {
        this.onPlay = callback
    }

    setUnsupported(message: string): void {
        this.button.textContent = message
        this.button.disabled = true
        this.button.style.backgroundColor = '#ed4337'
        this.button.style.cursor = 'not-allowed'
    }

    setReady(): void {
        this.button.textContent = 'Play with EasyMaps'
        this.button.disabled = false
        this.button.style.cursor = 'pointer'
        this.button.style.backgroundColor = '#24aae2'
    }

    setLoading(): void {
        this.button.textContent = 'Map is being installed. Please wait...'
        this.button.disabled = true
        this.button.style.cursor = 'wait'
    }

    setStarting(): void {
        this.button.textContent =
            'Minecraft version is being installed. Please wait...'
        this.button.disabled = true
        this.button.style.cursor = 'wait'
    }

    setFinished(): void {
        this.button.textContent = 'Minecraft will open in a few seconds!'
        this.button.disabled = true
        this.button.style.cursor = 'not-allowed'

        setTimeout(() => {
            this.button.textContent = 'Enjoy the adventure!'
        }, 8000)
    }

    setError(): void {
        this.button.textContent = 'An error occurred! Try again'
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
            this.setLoading()
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
