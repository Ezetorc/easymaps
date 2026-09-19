function run() {
  const { pathname } = window.location;

  if (!/^\/[^/]+\/?$/.test(pathname)) return;

  const $downloadButton = document.querySelector(".mc-download-btn");
  const $playButton = new EasyMapsPlayButton();

  $downloadButton.before($playButton.element);

  const isJavaEdition = !document.querySelector('a[href="/bedrock"]');

  if (!isJavaEdition) {
    $playButton.setUnsupported("EasyMaps doesn't support Bedrock yet... :(");
    return;
  }

  const isExternal =
    document.evaluate(
      '//p[contains(., "Opens in new tab")]',
      document,
      null,
      XPathResult.FIRST_ORDERED_NODE_TYPE,
      null
    ).singleNodeValue


  console.log(isExternal)

  if (isExternal) {
    $playButton.setUnsupported("EasyMaps doesn't work with external downloads yet... :(")
    return
  }

  const $minecraftVersion = document.querySelector('div[class="grid grid-cols-2 overflow-hidden rounded-[4px] border border-white/[0.09] bg-white/[0.025]"]')
  const minecraftVersion = $minecraftVersion.childNodes[1].childNodes[1].textContent

  $playButton.onClick(() => {
    const clickEvent = new Event("click", { bubbles: true });

    $downloadButton.dispatchEvent(clickEvent);

    browser.runtime.sendMessage({
      action: "StartDownload",
      minecraftVersion,
    });
  });

  browser.runtime.onMessage.addListener((message) => {
    console.log("[Browser > Adapter] Received: ", message);

    if (message.status == "Starting") {
      $playButton.setLoading();
    } else if (message.status == "Finished") {
      $playButton.setFinished();
    } else if (message.status == "Error" && "error" in message) {
      $playButton.setError(message.error);
    }
  });
}

run();
