import init, { run_protocol_demo, sigil_demo_version } from './pkg/sigil_core.js';

const $ = (id) => document.getElementById(id);
const encoder = new TextEncoder();
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

const runtime = document.querySelector('.runtime');
const runtimeLabel = $('runtimeLabel');
const input = $('messageInput');
const lossInput = $('lossInput');
const lossValue = $('lossValue');
const nodeToggle = $('nodeToggle');
const nodeInput = $('nodeInput');
const nodeValue = $('nodeValue');
const nodeControl = $('nodeControl');
const liveToggle = $('liveToggle');
const liveState = $('liveState');
const sendButton = $('sendButton');
const receiverText = $('receiverText');
const receiverState = $('receiverState');
const fragmentGrid = $('fragmentGrid');
const nodeMap = $('nodeMap');
const virtualNodes = $('virtualNodes');
const packetLayer = $('packetLayer');
const stages = [...document.querySelectorAll('.stage')];

let previousRun = null;
let running = false;
let pendingAutoRun = false;
let liveTimer = null;
let runSerial = 0;

function digestShort(value) {
  if (!value || value === 'none') return value;
  return `${value.slice(0, 18)}…${value.slice(-8)}`;
}

function updateByteCount() {
  const bytes = encoder.encode(input.value).length;
  $('byteCount').textContent = `${bytes} byte${bytes === 1 ? '' : 's'}`;
  $('byteCount').classList.toggle('over', bytes > 512);
}

function updateControls() {
  lossValue.textContent = lossInput.value;
  nodeValue.textContent = nodeInput.value;
  nodeControl.classList.toggle('disabled', !nodeToggle.checked);
  nodeInput.disabled = !nodeToggle.checked;
  liveState.textContent = liveToggle.checked ? 'LIVE' : 'MANUAL';
  liveState.classList.toggle('off', !liveToggle.checked);
}

function clearStageState() {
  stages.forEach((stage) => stage.classList.remove('active', 'done', 'skipped'));
  receiverText.classList.remove('revealed');
  receiverText.textContent = '—';
  receiverState.textContent = 'esperando reconstrucción autenticada';
  $('symbolStatus').textContent = 'esperando';
  $('cryptoStatus').textContent = 'esperando';
  $('fragmentStatus').textContent = 'esperando';
  $('nodeStatus').textContent = nodeToggle.checked ? 'esperando' : 'simulación desactivada';
  $('reconstructStatus').textContent = 'esperando';
  $('symbolPreview').textContent = '—';
  $('wirePreview').textContent = '—';
  $('reconstructPreview').textContent = '—';
  fragmentGrid.replaceChildren();
  virtualNodes.replaceChildren();
  packetLayer.replaceChildren();
  $('networkMetric').textContent = nodeToggle.checked ? `${nodeInput.value} virtuales` : 'local';
}

function renderFragments(result) {
  fragmentGrid.replaceChildren();
  for (const fragment of result.fragments) {
    const el = document.createElement('div');
    el.className = `fragment ${fragment.available ? 'available' : 'lost'}`;
    el.title = `cap ${fragment.capability} · digest ${fragment.payload_digest} · ${fragment.bytes} bytes`;
    const label = document.createElement('small');
    label.textContent = fragment.capability.slice(0, 6);
    el.append(label);
    fragmentGrid.append(el);
  }
}

function stableHash(value) {
  let hash = 2166136261;
  for (let i = 0; i < value.length; i += 1) {
    hash ^= value.charCodeAt(i);
    hash = Math.imul(hash, 16777619);
  }
  return hash >>> 0;
}

function renderVirtualNodes(result, nodeCount) {
  virtualNodes.replaceChildren();
  packetLayer.replaceChildren();

  const assignments = result.fragments.map((fragment, index) => {
    const nodeIndex = stableHash(`${fragment.capability}:${index}`) % nodeCount;
    return { fragment, index, nodeIndex };
  });

  const counts = Array.from({ length: nodeCount }, () => ({ total: 0, available: 0, lost: 0 }));
  assignments.forEach(({ fragment, nodeIndex }) => {
    counts[nodeIndex].total += 1;
    if (fragment.available) counts[nodeIndex].available += 1;
    else counts[nodeIndex].lost += 1;
  });

  counts.forEach((count, index) => {
    const node = document.createElement('div');
    node.className = 'virtual-node';
    node.dataset.nodeIndex = String(index);
    node.innerHTML = `<b>N${String(index + 1).padStart(2, '0')}</b><span>${count.total} shard${count.total === 1 ? '' : 's'}</span><small>${count.available} ok${count.lost ? ` · ${count.lost} lost` : ''}</small>`;
    if (count.lost && count.available) node.classList.add('partial');
    if (count.lost && !count.available) node.classList.add('failed');
    if (!count.total) node.classList.add('idle');
    virtualNodes.append(node);
  });

  return assignments;
}

function centerOf(element, rootRect) {
  const rect = element.getBoundingClientRect();
  return {
    x: rect.left - rootRect.left + rect.width / 2,
    y: rect.top - rootRect.top + rect.height / 2,
  };
}

async function animateVirtualTransport(assignments, serial) {
  await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve)));
  if (serial !== runSerial) return;

  const rootRect = nodeMap.getBoundingClientRect();
  const sender = centerOf(nodeMap.querySelector('.sim-endpoint.sender'), rootRect);
  const receiver = centerOf(nodeMap.querySelector('.sim-endpoint.receiver'), rootRect);
  const nodeEls = [...virtualNodes.querySelectorAll('.virtual-node')];
  const animations = [];

  assignments.forEach(({ fragment, nodeIndex, index }) => {
    const node = nodeEls[nodeIndex];
    const target = centerOf(node, rootRect);
    const packet = document.createElement('i');
    packet.className = `sim-packet ${fragment.available ? 'ok' : 'lost'}`;
    packet.title = `shard ${index + 1}`;
    packetLayer.append(packet);

    const start = { x: sender.x - 4, y: sender.y - 4 };
    const mid = { x: target.x - 4, y: target.y - 4 };
    const end = fragment.available
      ? { x: receiver.x - 4, y: receiver.y - 4 }
      : { x: target.x - 4, y: target.y - 4 };

    const delay = index * 28;
    const animation = packet.animate([
      { transform: `translate(${start.x}px, ${start.y}px) scale(.55)`, opacity: 0 },
      { transform: `translate(${start.x}px, ${start.y}px) scale(1)`, opacity: 1, offset: 0.08 },
      { transform: `translate(${mid.x}px, ${mid.y}px) scale(1)`, opacity: 1, offset: 0.55 },
      { transform: `translate(${end.x}px, ${end.y}px) scale(.72)`, opacity: fragment.available ? 0.92 : 0, offset: 0.93 },
      { transform: `translate(${end.x}px, ${end.y}px) scale(.35)`, opacity: 0 },
    ], {
      duration: fragment.available ? 760 : 610,
      delay,
      easing: 'cubic-bezier(.2,.7,.2,1)',
      fill: 'forwards',
    });

    animation.finished.then(() => {
      packet.remove();
      if (serial !== runSerial) return;
      node.classList.add(fragment.available ? 'touched' : 'dropped');
    }).catch(() => {});
    animations.push(animation.finished.catch(() => {}));
  });

  await Promise.all(animations);
}

async function revealStage(stageName, duration = 260, serial = runSerial) {
  const current = document.querySelector(`[data-stage="${stageName}"]`);
  if (!current || serial !== runSerial) return;
  current.classList.add('active');
  await wait(duration);
  if (serial !== runSerial) return;
  current.classList.remove('active');
  current.classList.add('done');
}

function scheduleLiveRun() {
  updateByteCount();
  clearTimeout(liveTimer);
  if (!liveToggle.checked) return;
  liveTimer = setTimeout(() => {
    if (running) {
      pendingAutoRun = true;
      return;
    }
    runDemo('live');
  }, 420);
}

async function runDemo(source = 'manual') {
  if (running) {
    if (source === 'live') pendingAutoRun = true;
    return;
  }

  const message = input.value;
  const byteLength = encoder.encode(message).length;
  if (!message.length) {
    input.focus();
    return;
  }
  if (byteLength > 512) {
    receiverState.textContent = 'límite de la demo superado: 512 bytes UTF-8';
    return;
  }

  running = true;
  pendingAutoRun = false;
  const serial = ++runSerial;
  const liveRun = source === 'live';
  sendButton.disabled = true;
  sendButton.textContent = liveRun ? 'Procesando…' : 'Ejecutando…';
  clearStageState();

  try {
    const started = performance.now();
    const raw = run_protocol_demo(message, Number(lossInput.value));
    const elapsed = performance.now() - started;
    const result = JSON.parse(raw);

    if (serial !== runSerial) return;

    $('epochLabel').textContent = `WASM ${elapsed.toFixed(2)} ms`;
    $('coreVersion').textContent = `v${result.version}`;
    $('wireBytes').textContent = `${result.outer_wire_bytes} B`;
    $('threshold').textContent = `${result.fragments_required}/${result.fragments_total}`;

    $('symbolStatus').textContent = `${result.symbol_count} símbolos internos · mapa nuevo`;
    $('symbolPreview').textContent = result.symbol_codes.length
      ? result.symbol_codes.slice(0, 3).map((value) => value.slice(0, 16)).join('  ·  ')
      : 'vacío';
    await revealStage('symbols', liveRun ? 115 : 260, serial);

    $('cryptoStatus').textContent = 'inner AEAD ✓ · outer AEAD ✓';
    $('wirePreview').textContent = `wire ${digestShort(result.outer_wire_digest)}`;
    await revealStage('crypto', liveRun ? 115 : 260, serial);

    renderFragments(result);
    $('fragmentStatus').textContent = `${result.fragments_total} generados · ${result.fragments_lost} perdidos · ${result.fragments_total - result.fragments_lost} disponibles`;
    await revealStage('fragments', liveRun ? 180 : 430, serial);

    if (nodeToggle.checked) {
      const nodeCount = Number(nodeInput.value);
      $('networkMetric').textContent = `${nodeCount} virtuales`;
      $('nodeStatus').textContent = `${nodeCount} nodos virtuales · ${result.fragments_total} rutas simuladas`;
      const assignments = renderVirtualNodes(result, nodeCount);
      const nodeStage = document.querySelector('[data-stage="nodes"]');
      nodeStage.classList.add('active');
      await animateVirtualTransport(assignments, serial);
      if (serial !== runSerial) return;
      nodeStage.classList.remove('active');
      nodeStage.classList.add('done');
      $('nodeStatus').textContent = `${result.fragments_total - result.fragments_lost} shards llegaron · ${result.fragments_lost} se descartaron`;
    } else {
      const nodeStage = document.querySelector('[data-stage="nodes"]');
      nodeStage.classList.add('skipped', 'done');
      $('nodeStatus').textContent = 'simulación desactivada · salto directo a reconstrucción local';
      $('networkMetric').textContent = 'local';
      await wait(liveRun ? 70 : 150);
    }

    $('reconstructStatus').textContent = result.reconstruction_matches
      ? 'ciphertext reconstruido · autenticación ✓'
      : 'reconstrucción inconsistente';
    $('reconstructPreview').textContent = `rebuilt ${digestShort(result.reconstructed_wire_digest)}`;
    await revealStage('reconstruct', liveRun ? 130 : 320, serial);

    if (serial !== runSerial) return;
    receiverText.textContent = result.receiver_text;
    receiverText.classList.add('revealed');
    receiverState.textContent = result.receiver_matches
      ? 'resultado autenticado · coincide con la entrada original'
      : 'el receptor no coincide';

    $('previousDigest').textContent = previousRun ? digestShort(previousRun.digest) : 'ninguno';
    $('currentDigest').textContent = digestShort(result.outer_wire_digest);

    if (previousRun && previousRun.message === message) {
      $('rotationState').textContent = previousRun.digest !== result.outer_wire_digest
        ? 'Mismo mensaje. Nuevo envelope confirmado.'
        : 'Inesperado: se repitió el digest del envelope.';
    } else if (previousRun) {
      $('rotationState').textContent = 'Cambió la entrada. Ejecutá este mismo texto otra vez para comparar.';
    } else {
      $('rotationState').textContent = 'Ejecutá el mismo mensaje otra vez para verificar la rotación.';
    }

    previousRun = { message, digest: result.outer_wire_digest };
  } catch (error) {
    console.error(error);
    receiverState.textContent = `error de protocolo: ${String(error)}`;
    $('epochLabel').textContent = 'error';
  } finally {
    if (serial === runSerial) {
      running = false;
      sendButton.disabled = false;
      sendButton.textContent = 'Ejecutar ahora';
    }
    if (pendingAutoRun && liveToggle.checked) {
      pendingAutoRun = false;
      setTimeout(() => runDemo('live'), 0);
    }
  }
}

input.addEventListener('input', scheduleLiveRun);
lossInput.addEventListener('input', () => {
  updateControls();
  scheduleLiveRun();
});
nodeInput.addEventListener('input', () => {
  updateControls();
  scheduleLiveRun();
});
nodeToggle.addEventListener('change', () => {
  updateControls();
  scheduleLiveRun();
});
liveToggle.addEventListener('change', () => {
  updateControls();
  if (liveToggle.checked) scheduleLiveRun();
  else clearTimeout(liveTimer);
});
sendButton.addEventListener('click', () => runDemo('manual'));
input.addEventListener('keydown', (event) => {
  if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') runDemo('manual');
});

try {
  await init();
  runtime.classList.add('ready');
  runtimeLabel.textContent = `Rust/WASM real · v${sigil_demo_version()}`;
  updateByteCount();
  updateControls();
  await runDemo('manual');
} catch (error) {
  console.error(error);
  runtimeLabel.textContent = 'WASM no pudo cargar';
  sendButton.disabled = true;
  receiverState.textContent = 'No se pudo inicializar el core WebAssembly.';
}
