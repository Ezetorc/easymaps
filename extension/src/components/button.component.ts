export class Button {
    private readonly _element: HTMLButtonElement
    private readonly _container: HTMLDivElement
    private readonly _menuButton: HTMLButtonElement
    private readonly _menu: HTMLDivElement

    private _onClick?: () => void
    private _actions: Map<string, () => void> = new Map()

    constructor() {
        this._container = document.createElement('div')
        this._container.className = 'x-button-wrapper'

        this._element = document.createElement('button')
        this._element.className = 'x-button x-button--idle'
        this._element.type = 'button'

        this._menuButton = document.createElement('button')
        this._menuButton.className = 'x-button__menu-trigger'
        this._menuButton.type = 'button'
        this._menuButton.textContent = '⋮'
        this._menuButton.setAttribute('aria-label', 'Más opciones')
        this._menuButton.setAttribute('aria-expanded', 'false')

        this._menu = document.createElement('div')
        this._menu.className = 'x-button__menu'
        this._menu.hidden = true

        this._container.append(this._element, this._menuButton, this._menu)

        this._element.addEventListener('click', () => {
            this._onClick?.()
        })

        this._menuButton.addEventListener('click', (event) => {
            event.stopPropagation()
            this.toggleMenu()
        })

        this._menu.addEventListener('click', (event) => {
            event.stopPropagation()
        })

        document.addEventListener('click', this._handleOutsideClick)
        document.addEventListener('keydown', this._handleKeydown)

        this._setupStyles()
    }

    get element() {
        return this._container
    }

    set onClick(callback: () => void) {
        this._onClick = callback
    }

    addAction(label: string, callback: () => void): void {
        this._actions.set(label, callback)
        this._renderActions()
    }

    removeAction(label: string): void {
        this._actions.delete(label)
        this._renderActions()
    }

    clearActions(): void {
        this._actions.clear()
        this._renderActions()
    }

    toggleMenu(): void {
        if (this._actions.size === 0) return

        const isOpen = !this._menu.hidden

        this._menu.hidden = isOpen
        this._menuButton.setAttribute('aria-expanded', String(!isOpen))
    }

    private _closeMenu(): void {
        this._menu.hidden = true
        this._menuButton.setAttribute('aria-expanded', 'false')
    }

    private _handleOutsideClick = (event: MouseEvent): void => {
        if (!this._container.contains(event.target as Node)) {
            this._closeMenu()
        }
    }

    private _handleKeydown = (event: KeyboardEvent): void => {
        if (event.key === 'Escape') {
            this._closeMenu()
        }
    }

    private _renderActions(): void {
        this._menu.replaceChildren()

        this._menuButton.hidden = this._actions.size === 0

        for (const [label, callback] of this._actions) {
            const button = document.createElement('button')

            button.type = 'button'
            button.className = 'x-button__menu-item'
            button.textContent = label

            button.addEventListener('click', () => {
                callback()
                this._closeMenu()
            })

            this._menu.appendChild(button)
        }

        if (this._actions.size === 0) {
            this._closeMenu()
        }
    }

    private _updateMenuAvailability(state: string): void {
        const disabledStates = ['disabled', 'warning', 'error', 'loading']
        const isDisabled = disabledStates.includes(state)

        this._menuButton.disabled = isDisabled

        if (isDisabled) {
            this._closeMenu()
        }

        this._menuButton.style.opacity = isDisabled ? '0.5' : '1'
        this._menuButton.style.cursor = isDisabled ? 'not-allowed' : 'pointer'
    }

    setIdle(message?: string): void {
        this._setState('idle', message, false)
    }

    setLoading(message?: string): void {
        this._setState('loading', message, true)
    }

    setSuccess(message?: string): void {
        this._setState('success', message, true)
    }

    setError(message?: string): void {
        this._setState('error', message, true)
    }

    setWarning(message?: string): void {
        this._setState('warning', message, true)
    }

    setDisabled(message?: string): void {
        this._setState('disabled', message, true)
    }

    private _setState(state: string, message?: string, disabled = false): void {
        this._element.className = `x-button x-button--${state}`
        this._element.disabled = disabled

        this._updateMenuAvailability(state)

        if (message !== undefined) {
            this._element.textContent = message
        }
    }

    private _setupStyles(): void {
        if (document.getElementById('x-button-style')) return

        const style = document.createElement('style')
        style.id = 'x-button-style'

        style.textContent = `
            .x-button-wrapper {
                position: relative;
                display: flex;
                width: 100%;
                margin-bottom: 16px;
                isolation: auto;
            }

            .x-button {
                flex: 1;
                min-width: 0;
                padding: 1rem 1.5rem;

                border: none;
                border-radius: 2px 0 0 2px;

                color: #fff;
                font-size: 1rem;
                font-weight: 800;

                transition:
                    transform 0.2s ease,
                    box-shadow 0.2s ease,
                    background-color 0.2s ease,
                    opacity 0.2s ease;
            }

            .x-button--idle {
                background-color: #2563EB;
                box-shadow: 0 2px 4px #2563EB33;
                cursor: pointer;
            }

            .x-button--idle:hover {
                background-color: #1D4ED8;
                box-shadow: 0 4px 8px #2563EB40;
            }

            .x-button--idle:active {
                background-color: #1E40AF;
                transform: scale(0.98);
            }

            .x-button--loading {
                background-color: #64748B;
                cursor: wait;
                opacity: 0.9;
            }

            .x-button--success {
                background-color: #16A34A;
                box-shadow: 0 2px 4px #16A34A33;
                cursor: not-allowed;
            }

            .x-button--success:hover {
                background-color: #15803D;
            }

            .x-button--error {
                background-color: #DC2626;
                box-shadow: 0 2px 4px #DC262633;
                cursor: not-allowed;
            }

            .x-button--error:hover {
                background-color: #B91C1C;
            }

            .x-button--warning {
                background-color: #EA580C;
                box-shadow: 0 2px 4px #EA580C33;
                cursor: not-allowed;
            }

            .x-button--warning:hover {
                background-color: #C2410C;
            }

            .x-button--disabled {
                background-color: #D1D5DB;
                color: #374151;
                cursor: not-allowed;
                opacity: 0.7;
                box-shadow: none;
            }

            .x-button:disabled {
                pointer-events: none;
            }

            .x-button__menu-trigger {
                width: 48px;
                flex-shrink: 0;

                border: none;
                border-left: 1px solid #FFFFFF33;
                border-radius: 0 6px 6px 0;

                background-color: #1D4ED8;
                color: #FFFFFF;

                font-size: 1.5rem;
                font-weight: bold;

                cursor: pointer;
                transition: background-color 0.2s ease;
            }

            .x-button__menu-trigger:hover {
                background-color: #1E40AF;
            }

            .x-button__menu-trigger[hidden] {
                display: none;
            }

            .x-button__menu {
                position: absolute;
                z-index: 9999;
                top: calc(100% + 4px);
                right: 0;

                min-width: 180px;
                padding: 5px;

                background: #FFFFFF;
                border: 1px solid #E5E7EB;
                border-radius: 8px;

                box-shadow: 0 4px 12px #00000020;
            }

            .x-button__menu[hidden] {
                display: none;
            }

            .x-button__menu-item {
                display: block;
                width: 100%;
                z-index: 9999;

                padding: 10px 12px;

                border: none;
                border-radius: 5px;

                background: transparent;
                color: #374151;

                font-size: 0.9rem;
                text-align: left;

                cursor: pointer;
                transition: background-color 0.15s ease;
            }

            .x-button__menu-item:hover {
                background-color: #F3F4F6;
            }
        `

        document.head.appendChild(style)
    }

    destroy(): void {
        document.removeEventListener('click', this._handleOutsideClick)
        document.removeEventListener('keydown', this._handleKeydown)
        this._container.remove()
    }
}
