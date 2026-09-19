class EasyMapsPlayButton {
  constructor() {
    this.element = document.createElement("button");

    this.element.textContent = "Play me NOW!";
    this.element.style.width = "100%";
    this.element.style.height = "64px";
    this.element.style.marginBottom = "16px";
    this.element.style.cursor = "pointer";
    this.element.style.backgroundColor = "blue";

    this.setReady();

    this.element.addEventListener("click", () => {
      this.onPlay?.();
    });
  }

  onClick(callback) {
    this.onPlay = callback;
  }

  setUnsupported(message) {
    this.element.textContent = message;

    this.element.disabled = true;
    this.element.style.cursor = "not-allowed";
  }

  setLoading() {
    this.element.textContent = "Please wait...";
    this.element.disabled = true;
  }

  setReady() {
    this.element.textContent = "Play me NOW!";
    this.element.disabled = false;
    this.element.style.cursor = "pointer";
  }

  setFinished() {
    this.element.textContent = "Enjoy the adventure!";
    this.element.disabled = true;
    this.element.style.cursor = "not-allowed";
  }

  setError(message) {
    this.element.textContent = `An error ocurred: '${message}'`;
    this.element.disabled = true;
    this.element.style.cursor = "not-allowed";
  }
}
