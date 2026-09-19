let port = browser.runtime.connectNative("com.easymaps.host");
let clickedPlayButton = false;
let downloads = new Map();
let playTabId = null;
let minecraftVersion = null;

port.onMessage.addListener((message) => {
  console.log("[Port] Message received: ", message);

  if (playTabId != null) {
    browser.tabs.sendMessage(playTabId, message);
  }
});

port.onDisconnect.addListener(() => {
  console.warn("[Port] Port disconnected");
});

browser.runtime.onMessage.addListener(async (message, sender) => {
  console.log("[Browser > Background] Message received: ", message);

  if ("tab" in sender) {
    playTabId = sender.tab.id;
  }

  if ("action" in message) {
    if (message.action == "StartDownload") {
      clickedPlayButton = true;

      if ("minecraftVersion" in message) {
        minecraftVersion = message.minecraftVersion;
      }
    }
  }
});

browser.downloads.onCreated.addListener((download) => {
  console.log(
    `[Browser > Background] Download created: [${download.id}]: ${download.filename}`,
  );

  if (clickedPlayButton) {
    downloads.set(download.id, { filename: download.filename });

    clickedPlayButton = false;
  }
});

browser.downloads.onChanged.addListener((changedDownload) => {
  console.log(
    `[Browser > Background] Download changed: [${changedDownload.id}]: ${changedDownload.state.current}`,
  );

  for (const [id, download] of downloads.entries()) {
    if (
      id === changedDownload.id &&
      changedDownload.state.current === "complete"
    ) {
      downloads.delete(id);

      port.postMessage({
        action: "Start",
        minecraft_version: minecraftVersion,
        filename: download.filename,
      });
    }
  }
});
