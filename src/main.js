const { invoke } = window.__TAURI__.core;
const { open, save } = window.__TAURI__.dialog;

let selectedPaths = [];

const selectBtn = document.getElementById("select-btn");
const convertBtn = document.getElementById("convert-btn");
const clearBtn = document.getElementById("clear-btn");
const preview = document.getElementById("preview");
const statusEl = document.getElementById("status");

function setStatus(msg, isError = false) {
  statusEl.textContent = msg;
  statusEl.className = "status " + (isError ? "error" : "success");
}

function updateUI() {
  const hasFiles = selectedPaths.length > 0;
  convertBtn.disabled = !hasFiles;
  clearBtn.disabled = !hasFiles;

  if (!hasFiles) {
    preview.innerHTML = `<p class="empty">Henüz resim seçilmedi</p>`;
    return;
  }

  preview.innerHTML = selectedPaths
    .map((p, i) => {
      const name = p.split(/[\\/]/).pop();
      return `<div class="file-item">
        <span class="index">${i + 1}.</span>
        <span class="name" title="${p}">${name}</span>
      </div>`;
    })
    .join("");
}

selectBtn.addEventListener("click", async () => {
  try {
    const files = await open({
      multiple: true,
      filters: [
        {
          name: "Images",
          extensions: ["png", "jpg", "jpeg", "gif", "bmp", "webp", "tiff", "tif"],
        },
      ],
    });

    if (files === null) return; // user cancelled

    // files can be string or string[]
    if (Array.isArray(files)) {
      selectedPaths = [...selectedPaths, ...files];
    } else if (typeof files === "string") {
      selectedPaths.push(files);
    }

    // dedupe
    selectedPaths = [...new Set(selectedPaths)];
    updateUI();
    setStatus(`${selectedPaths.length} resim seçildi`);
  } catch (err) {
    setStatus("Seçim hatası: " + err, true);
  }
});

clearBtn.addEventListener("click", () => {
  selectedPaths = [];
  updateUI();
  setStatus("");
});

convertBtn.addEventListener("click", async () => {
  if (selectedPaths.length === 0) return;

  try {
    convertBtn.disabled = true;
    setStatus("PDF oluşturuluyor...");

    const output = await save({
      filters: [{ name: "PDF", extensions: ["pdf"] }],
      defaultPath: "images.pdf",
    });

    if (!output) {
      setStatus("Kaydetme iptal edildi");
      convertBtn.disabled = false;
      return;
    }

    const result = await invoke("convert_images_to_pdf", {
      imagePaths: selectedPaths,
      outputPath: output,
    });

    setStatus(result);
  } catch (err) {
    setStatus("Hata: " + (err?.message || err), true);
  } finally {
    convertBtn.disabled = selectedPaths.length === 0;
  }
});

window.addEventListener("DOMContentLoaded", () => {
  updateUI();
});
