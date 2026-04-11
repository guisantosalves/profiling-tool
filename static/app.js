const $ = (id) => document.getElementById(id);

const statusDot = $("ws-status");
const statusLabel = $("ws-label");

function setStatus(state) {
  statusDot.className = "dot " + state;
  if (state === "ok") statusLabel.textContent = "conectado";
  else if (state === "err") statusLabel.textContent = "desconectado";
  else statusLabel.textContent = "conectando...";
}

function formatBytes(bytes) {
  if (bytes === 0 || bytes == null) return "0 B";
  const k = 1024;
  const units = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return (bytes / Math.pow(k, i)).toFixed(1) + " " + units[i];
}

function formatUptime(seconds) {
  const d = Math.floor(seconds / 86400);
  const h = Math.floor((seconds % 86400) / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = Math.floor(seconds % 60);
  if (d > 0) return `${d}d ${h}h ${m}m`;
  if (h > 0) return `${h}h ${m}m ${s}s`;
  if (m > 0) return `${m}m ${s}s`;
  return `${s}s`;
}

function severity(pct) {
  if (pct >= 85) return "danger";
  if (pct >= 65) return "warn";
  return "";
}

function setBar(id, pct) {
  const el = $(id);
  const clamped = Math.max(0, Math.min(100, pct));
  el.style.width = clamped + "%";
  el.className = "bar-fill " + severity(clamped);
}

function renderHeader(stats) {
  $("host-name").textContent = stats.system.host_name || "—";
  $("os-name").textContent = stats.system.name || "—";
  $("kernel-version").textContent = stats.system.kernel_version || "—";
  $("uptime").textContent = formatUptime(stats.uptime);
}

function renderCpu(stats) {
  const pct = stats.cpu_usage;
  $("cpu-global").textContent = pct.toFixed(1);
  setBar("cpu-bar", pct);

  const logical = stats.cpus.length;
  const physical = stats.physical_cores ?? "?";
  $("cpu-meta").textContent = `${physical} núcleos · ${logical} threads`;

  const container = $("cpu-cores");
  if (container.children.length !== stats.cpus.length) {
    container.innerHTML = "";
    stats.cpus.forEach((_, i) => {
      const div = document.createElement("div");
      div.className = "core";
      div.innerHTML = `
        <div class="core-header">
          <span>Core ${i}</span>
          <span class="core-value">0%</span>
        </div>
        <div class="bar"><div class="bar-fill"></div></div>
      `;
      container.appendChild(div);
    });
  }

  stats.cpus.forEach((core, i) => {
    const node = container.children[i];
    const value = core.usage.toFixed(0);
    node.querySelector(".core-value").textContent = value + "%";
    const fill = node.querySelector(".bar-fill");
    fill.style.width = value + "%";
    fill.className = "bar-fill " + severity(core.usage);
  });
}

function renderMemory(stats) {
  const memPct = (stats.used_memory / stats.total_memory) * 100;
  $("mem-percent").textContent = memPct.toFixed(1);
  setBar("mem-bar", memPct);
  $("mem-meta").textContent = `${formatBytes(stats.used_memory)} / ${formatBytes(stats.total_memory)}`;

  if (stats.total_swap > 0) {
    const swapPct = (stats.used_swap / stats.total_swap) * 100;
    setBar("swap-bar", swapPct);
    $("swap-meta").textContent = `${formatBytes(stats.used_swap)} / ${formatBytes(stats.total_swap)}`;
  } else {
    setBar("swap-bar", 0);
    $("swap-meta").textContent = "sem swap";
  }
}

function renderDisks(stats) {
  const container = $("disks-list");
  container.innerHTML = "";
  stats.disks.forEach((d) => {
    const used = d.total_space - d.available_space;
    const pct = d.total_space > 0 ? (used / d.total_space) * 100 : 0;
    const div = document.createElement("div");
    div.className = "disk";
    div.innerHTML = `
      <div class="disk-header">
        <span class="disk-name">${d.mount_point || d.name}</span>
        <span class="disk-meta">${formatBytes(used)} / ${formatBytes(d.total_space)} · ${pct.toFixed(0)}%</span>
      </div>
      <div class="bar"><div class="bar-fill ${severity(pct)}" style="width:${pct}%"></div></div>
    `;
    container.appendChild(div);
  });
}

function renderTemperature(stats) {
  if (stats.temperature == null) {
    $("temperature").textContent = "—";
    $("temperature-unit").textContent = "";
  } else {
    $("temperature").textContent = stats.temperature.toFixed(1);
    $("temperature-unit").textContent = "°C";
  }
}

let procFilter = "";
let procPaused = false;
let lastProcesses = [];

const searchInput = $("proc-search");
const pauseBtn = $("proc-pause");

searchInput.addEventListener("input", (e) => {
  procFilter = e.target.value.trim().toLowerCase();
  renderProcessList();
});

pauseBtn.addEventListener("click", () => {
  procPaused = !procPaused;
  pauseBtn.textContent = procPaused ? "▶" : "⏸";
  pauseBtn.classList.toggle("paused", procPaused);
  pauseBtn.title = procPaused ? "Retomar atualizações" : "Pausar atualizações";
});

async function killProcess(pid, btn) {
  if (!confirm(`Matar processo ${pid}?`)) return;
  btn.disabled = true;
  btn.textContent = "...";
  try {
    const res = await fetch(`/process/${pid}/kill`, { method: "POST" });
    if (!res.ok) {
      btn.textContent = "falhou";
      btn.classList.add("btn-fail");
      setTimeout(() => {
        btn.disabled = false;
        btn.textContent = "kill";
        btn.classList.remove("btn-fail");
      }, 1500);
    }
  } catch (err) {
    btn.textContent = "erro";
    btn.disabled = false;
  }
}

function renderProcessList() {
  const body = $("proc-body");
  const count = $("proc-count");
  body.innerHTML = "";

  const filtered = procFilter
    ? lastProcesses.filter(
        (p) => p.name.toLowerCase().includes(procFilter) || String(p.pid).includes(procFilter)
      )
    : lastProcesses;

  count.textContent = procFilter
    ? `${filtered.length} de ${lastProcesses.length} processos`
    : `Top ${lastProcesses.length} por CPU`;

  filtered.forEach((p) => {
    const tr = document.createElement("tr");
    tr.innerHTML = `
      <td>${p.pid}</td>
      <td>${p.name}</td>
      <td class="num">${p.cpu_usage.toFixed(1)}%</td>
      <td class="num">${formatBytes(p.memory)}</td>
      <td class="num"><button class="btn-kill" data-pid="${p.pid}">kill</button></td>
    `;
    body.appendChild(tr);
  });
  body.querySelectorAll(".btn-kill").forEach((btn) => {
    btn.addEventListener("click", () => killProcess(btn.dataset.pid, btn));
  });
}

function renderProcesses(stats) {
  if (procPaused) return;
  lastProcesses = stats.processes;
  renderProcessList();
}

function render(stats) {
  renderHeader(stats);
  renderCpu(stats);
  renderMemory(stats);
  renderDisks(stats);
  renderTemperature(stats);
  renderProcesses(stats);
}

function connect() {
  setStatus("");
  const proto = location.protocol === "https:" ? "wss" : "ws";
  const ws = new WebSocket(`${proto}://${location.host}/ws`);

  ws.onopen = () => setStatus("ok");
  ws.onmessage = (e) => {
    try {
      render(JSON.parse(e.data));
    } catch (err) {
      console.error("parse error", err);
    }
  };
  ws.onclose = () => {
    setStatus("err");
    setTimeout(connect, 2000);
  };
  ws.onerror = () => ws.close();
}

connect();
