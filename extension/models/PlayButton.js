class EasyMapsPlayButton {
  constructor() {
    this.element = document.createElement("button");

    this.element.style.width = "100%";
    this.element.style.padding = "1rem 1.5rem";
    this.element.style.marginBottom = "16px";
    this.element.style.backgroundColor = "#24aae2";
    this.element.style.borderRadius = "4px"
    this.element.style.color = "black"
    this.element.style.fontWeight = "bold"

    this.setReady();

    this.element.addEventListener("click", () => {
      this.onPlay?.();
      this.setLoading()
    });

    this.element.style.transition =
      "transform 0.2s ease, box-shadow 0.2s ease, background-color 0.2s ease";

    this.element.addEventListener("mouseenter", () => {
      if (this.element.disabled) return;

      this.element.style.transform = "translateY(-3px)";
      this.element.style.boxShadow = "0 6px 18px rgba(36, 170, 226, 0.4)";
      this.element.style.backgroundColor = "#35b8ed";
    });

    this.element.addEventListener("mouseleave", () => {
      if (this.element.disabled) return;

      this.element.style.transform = "translateY(0)";
      this.element.style.boxShadow = "0 2px 6px rgba(0, 0, 0, 0.15)";
      this.element.style.backgroundColor = "#24aae2";
    });

    this.element.addEventListener("mousedown", () => {
      if (this.element.disabled) return;

      this.element.style.transform = "translateY(-1px) scale(0.98)";
    });

    this.element.addEventListener("mouseup", () => {
      if (this.element.disabled) return;

      this.element.style.transform = "translateY(-3px)";
    });
  }

  onClick(callback) {
    this.onPlay = callback;
  }

  setUnsupported(message) {
    this.element.textContent = message;
    this.element.disabled = true;
    this.element.style.backgroundColor = "#ed4337"
    this.element.style.cursor = "not-allowed";
  }

  setReady() {
    this.element.textContent = "Play with EasyMaps";
    this.element.disabled = false;
    this.element.style.cursor = "pointer";
    this.element.style.backgroundColor = "#24aae2"
  }

  setLoading() {
    this.element.textContent = "Map is being installed. Please wait...";
    this.element.disabled = true;
    this.element.style.cursor = "wait";
  }

  setStarting() {
    this.element.textContent = "Minecraft version is being installed. Please wait...";
    this.element.disabled = true;
    this.element.style.cursor = "wait";
  }

  setFinished() {
    this.element.textContent = "Minecraft will open in a few seconds!";
    this.element.disabled = true;
    this.element.style.cursor = "not-allowed";

    setTimeout(() => {
      this.element.textContent = "Enjoy the adventure!";

    }, 8000)
  }

  setError(message) {
    this.element.textContent = `An error ocurred: '${message}'`;
    this.element.disabled = true;
    this.element.style.cursor = "not-allowed";
  }
}
