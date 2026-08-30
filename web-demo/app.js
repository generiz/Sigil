import init, { run_protocol_demo, sigil_demo_version } from './pkg/sigil_core.js';

const $ = (id) => document.getElementById(id);
const encoder = new TextEncoder();
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

const runtime = document.querySelector('.runtime');
const runtimeLabel = $('runtimeLabel');
const input = $('messageInput');
const lossInput = $('lossInput');
const lossValue = $('lossValue');
const sendButton = $('sendButton');
const receiverText = $('receiverText');
const receiverState = $('receiverState');
const fragmentGrid = $('fragmentGrid');
const stages = [...document.querySelectorAll('.stage')];

let previousRun = null;
let running = false;

function digestShort(value) {
  if (!value || value === 'none') return value;
  return `${value.slice(0, 18)}…${value.slice(-8)}`;
}

function updateByteCount() {
  const bytes = encoder.encode(input.value).length;
  $('byteCount').textContent = `${bytes} byte${bytes === 1 ? '' : 's'}`;
  $('byteCount').classList.toggle('over', bytes > 512);
}

function clearStageState() {
  stages.forEach((stage) => stage.classList.remove('active', 'done'));
  receiverText.classList.remove('revealed');
  receiverText.textContent = '—';
  receiverState.textContent = 'waiting for authenticated reconstruction';
  $('symbolStatus').textContent = 'waiting';
  $('cryptoStatus').textContent = 'waiting';
  $('fragmentStatus').textContent = 'waiting';
  $('reconstructStatus').textContent = 'waiting';
  $('symbolPreview').textContent = '—';
  $('wirePreview').textContent = '—';
  $('reconstructPreview').textContent = '—';
  fragmentGrid.replaceChildren();
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

async function revealStage(stageName, duration = 380) {
  const current = document.querySelector(`[data-stage="${stageName}"]`);
  current.classList.add('active');
  await wait(duration);
  current.classList.remove('active');
  current.classList.add('done');
}

async function runDemo() {
  if (running) return;
  const message = input.value;
  const byteLength = encoder.encode(message).length;
  if (!message.length) {
    input.focus();
    return;
  }
  if (byteLength > 512) {
    receiverState.textContent = 'browser demo limit exceeded: 512 UTF-8 bytes';
    return;
  }

  running = true;
  sendButton.disabled = true;
  sendButton.textContent = 'Running…';
  clearStageState();

  try {
    const started = performance.now();
    const raw = run_protocol_demo(message, Number(lossInput.value));
    const elapsed = performance.now() - started;
    const result = JSON.parse(raw);

    $('epochLabel').textContent = `WASM ${elapsed.toFixed(2)} ms`;
    $('coreVersion').textContent = `v${result.version}`;
    $('wireBytes').textContent = `${result.outer_wire_bytes} B`;
    $('threshold').textContent = `${result.fragments_required}/${result.fragments_total}`;

    $('symbolStatus').textContent = `${result.symbol_count} internal symbols · fresh map`;
    $('symbolPreview').textContent = result.symbol_codes.length
      ? result.symbol_codes.slice(0, 3).map((value) => value.slice(0, 16)).join('  ·  ')
      : 'empty';
    await revealStage('symbols');

    $('cryptoStatus').textContent = 'inner AEAD ✓ · outer AEAD ✓';
    $('wirePreview').textContent = `wire ${digestShort(result.outer_wire_digest)}`;
    await revealStage('crypto');

    renderFragments(result);
    $('fragmentStatus').textContent = `${result.fragments_total} generated · ${result.fragments_lost} unavailable · ${result.fragments_total - result.fragments_lost} retained`;
    await revealStage('fragments', 650);

    $('reconstructStatus').textContent = result.reconstruction_matches
      ? 'ciphertext rebuilt · authentication ✓'
      : 'reconstruction mismatch';
    $('reconstructPreview').textContent = `rebuilt ${digestShort(result.reconstructed_wire_digest)}`;
    await revealStage('reconstruct', 520);

    receiverText.textContent = result.receiver_text;
    receiverText.classList.add('revealed');
    receiverState.textContent = result.receiver_matches
      ? 'authenticated result · matches original browser input'
      : 'receiver mismatch';

    $('previousDigest').textContent = previousRun ? digestShort(previousRun.digest) : 'none';
    $('currentDigest').textContent = digestShort(result.outer_wire_digest);

    if (previousRun && previousRun.message === message) {
      $('rotationState').textContent = previousRun.digest !== result.outer_wire_digest
        ? 'Same message. New outer envelope confirmed.'
        : 'Unexpected: envelope digest repeated.';
    } else if (previousRun) {
      $('rotationState').textContent = 'Input changed. Run this exact message again to compare.';
    } else {
      $('rotationState').textContent = 'Run the same message again to verify rotation.';
    }

    previousRun = { message, digest: result.outer_wire_digest };
  } catch (error) {
    console.error(error);
    receiverState.textContent = `protocol error: ${String(error)}`;
    $('epochLabel').textContent = 'error';
  } finally {
    running = false;
    sendButton.disabled = false;
    sendButton.textContent = 'Run protocol';
  }
}

input.addEventListener('input', updateByteCount);
lossInput.addEventListener('input', () => {
  lossValue.textContent = lossInput.value;
});
sendButton.addEventListener('click', runDemo);
input.addEventListener('keydown', (event) => {
  if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') runDemo();
});

try {
  await init();
  runtime.classList.add('ready');
  runtimeLabel.textContent = `Rust/WASM ready · v${sigil_demo_version()}`;
  updateByteCount();
  await runDemo();
} catch (error) {
  console.error(error);
  runtimeLabel.textContent = 'WASM failed to load';
  sendButton.disabled = true;
  receiverState.textContent = 'The WebAssembly core could not be initialized.';
}
