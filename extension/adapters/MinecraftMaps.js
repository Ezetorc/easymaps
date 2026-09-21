class MinecraftMapsAdapter extends AbstractWebAdapter {
    isValidPage() {
        const { pathname } = window.location;

        return /^\/[^/]+\/?$/.test(pathname)
    }

    createPlayButton() {
        this.playButton = new EasyMapsPlayButton();
    }

    isValidMap() {
        const isJavaEdition = !document.querySelector('a[href="/bedrock"]');

        if (!isJavaEdition) {
            this.playButton.setUnsupported("EasyMaps doesn't support Bedrock yet...");
            return false;
        }

        const isModded = document.querySelector('a[href="/modded"]');

        if (isModded) {
            this.playButton.setUnsupported("EasyMaps doesn't support modded maps yet...");
            return false;
        }

        const isExternal =
            document.evaluate(
                '//p[contains(., "Opens in new tab")]',
                document,
                null,
                XPathResult.FIRST_ORDERED_NODE_TYPE,
                null
            ).singleNodeValue

        if (isExternal) {
            this.playButton.setUnsupported("EasyMaps doesn't work with external downloads yet...")
            return false
        }

        return true
    }

    addPlayButton() {
        const $downloadButton = document.querySelector(".mc-download-btn");

        $downloadButton.before(this.playButton.element);
    }

    determineMinecraftVersion() {
        const $minecraftVersion = document.querySelector('div[class="grid grid-cols-2 overflow-hidden rounded-[4px] border border-white/[0.09] bg-white/[0.025]"]')
        const minecraftVersion = $minecraftVersion.childNodes[1].childNodes[1].textContent

        return minecraftVersion
    }

    addDownloadEvent(minecraftVersion) {
        this.playButton.onClick(() => {
            const clickEvent = new Event("click", { bubbles: true });
            const $downloadButton = document.querySelector(".mc-download-btn");

            $downloadButton.dispatchEvent(clickEvent);

            browser.runtime.sendMessage({
                action: "StartDownload",
                minecraftVersion,
            });
        });
    }

    addResponseEvent() {
        browser.runtime.onMessage.addListener((message) => {
            console.log("[Browser > Adapter] Received: ", message);

            if (message.status === "Starting") {
                this.playButton.setStarting();
            } else if (message.status === "Finished") {
                this.playButton.setFinished();
            } else if (message.status === "Error" && "error" in message) {
                this.playButton.setError(message.error);
            }
        });
    }
}

new MinecraftMapsAdapter().run();