from pathlib import Path

path = Path('web-demo/embed.js')
text = path.read_text(encoding='utf-8')

replacements = []

replacements.append((
"""    this.ready = false;\n    this.timer = null;\n""",
"""    this.ready = false;\n    this.timer = null;\n    this.playback = 0;\n"""
))

replacements.append((
"""    if (this.session?.free) this.session.free();\n    clearTimeout(this.timer);\n""",
"""    if (this.session?.free) this.session.free();\n    clearTimeout(this.timer);\n    this.playback += 1;\n"""
))

replacements.append((
"""        .controls{border-top:1px solid var(--line);padding:13px 17px;display:grid;gap:10px}.range{display:grid;grid-template-columns:1fr 60px;gap:7px;align-items:center}.range span{color:#667169;font:600 8px/1 ui-monospace,SFMono-Regular,Menlo,monospace;text-transform:uppercase;letter-spacing:.08em}.range input{width:100%;accent-color:#849b88}.range b{text-align:right;color:#acb7af;font-size:13px}.actions{display:grid;grid-template-columns:1fr 1fr;gap:7px}.action{border:1px solid #3a453d;border-radius:999px;background:#0c110e;color:#9aa69d;padding:10px 11px;cursor:pointer;font-size:9px}.action.primary{background:#e6ebe6;color:#080a09;border-color:#e6ebe6;font-weight:700}.action:disabled{opacity:.35;cursor:default}\n""",
"""        .controls{border-top:1px solid var(--line);padding:13px 17px;display:grid;gap:10px}.range{display:grid;grid-template-columns:1fr 60px;gap:7px;align-items:center}.range span{color:#667169;font:600 8px/1 ui-monospace,SFMono-Regular,Menlo,monospace;text-transform:uppercase;letter-spacing:.08em}.range input{width:100%;accent-color:#849b88}.range b{text-align:right;color:#acb7af;font-size:13px}.actions{display:grid;grid-template-columns:1fr 1fr;gap:7px}.action{border:1px solid #3a453d;border-radius:999px;background:#0c110e;color:#9aa69d;padding:10px 11px;cursor:pointer;font-size:9px}.action.primary{background:#e6ebe6;color:#080a09;border-color:#e6ebe6;font-weight:700}.action:disabled{opacity:.35;cursor:default}\n        .guide{border-bottom:1px solid var(--line);padding:14px 17px;background:#080c09;display:grid;gap:9px}.guide-head{display:flex;align-items:center;justify-content:space-between;gap:14px}.guide-head b,.guide-head span{font:650 7px/1 ui-monospace,SFMono-Regular,Menlo,monospace;letter-spacing:.1em;text-transform:uppercase}.guide-head b{color:#9aab9e}.guide-head span{color:#617067}.guide-copy strong{display:block;color:#d9e0da;font-size:12px;margin-bottom:5px}.guide-copy p{margin:0;color:#78857c;font-size:10px;line-height:1.55;max-width:680px}.guide-progress{height:2px;background:#1e2721;overflow:hidden;border-radius:2px}.guide-progress i{display:block;width:0;height:100%;background:#8da292;transition:width .25s ease}.guide-note{color:#566159;font:550 7px/1.45 ui-monospace,SFMono-Regular,Menlo,monospace}.guide-note b{color:#7f9084}\n"""
))

replacements.append((
"""              <label class=\"range\"><span>shards perdidos</span><input id=\"loss\" type=\"range\" min=\"0\" max=\"8\" value=\"0\"><b id=\"lossValue\">0</b></label>\n              <div class=\"actions\"><button id=\"fail\" class=\"action\">Matar nodo</button><button id=\"restore\" class=\"action\">Restaurar</button></div>\n              <div class=\"actions\"><button id=\"rerun\" class=\"action primary\">Nuevo envelope</button><button id=\"clear\" class=\"action\">Borrar mensaje</button></div>\n""",
"""              <label class=\"range\"><span>shards perdidos</span><input id=\"loss\" type=\"range\" min=\"0\" max=\"8\" value=\"0\"><b id=\"lossValue\">0</b></label>\n              <label class=\"range\"><span>pausa visual al enviar</span><input id=\"delay\" type=\"range\" min=\"0\" max=\"2000\" step=\"100\" value=\"900\"><b id=\"delayValue\">0.9 s</b></label>\n              <div class=\"actions\"><button id=\"fail\" class=\"action\">Matar nodo</button><button id=\"restore\" class=\"action\">Restaurar</button></div>\n              <div class=\"actions\"><button id=\"rerun\" class=\"action primary\">Nuevo envelope</button><button id=\"replay\" class=\"action\" disabled>Ver paso a paso</button></div>\n              <button id=\"clear\" class=\"action\">Borrar mensaje</button>\n"""
))

replacements.append((
"""          <section class=\"pane pipeline\">\n            <div class=\"pane-head\"><span>02</span> Live pipeline</div>\n            <div class=\"stages\">\n""",
"""          <section class=\"pane pipeline\">\n            <div class=\"pane-head\"><span>02</span> Live pipeline</div>\n            <div class=\"guide\">\n              <div class=\"guide-head\"><b id=\"guideStep\">RECORRIDO EN VIVO</b><span id=\"guideTiming\">0.9 s / etapa</span></div>\n              <div class=\"guide-copy\"><strong id=\"guideTitle\">Qué hace Sigil</strong><p id=\"guideText\">Mientras escribís, el core actualiza rápido. Al tocar ENVIAR o “Ver paso a paso”, esta vista ralentiza cada etapa para explicar qué ocurre.</p></div>\n              <div class=\"guide-progress\"><i id=\"guideBar\"></i></div>\n              <div class=\"guide-note\"><b>Importante:</b> el delay es solo visual. El cifrado y la reconstrucción siguen ejecutándose a velocidad real.</div>\n            </div>\n            <div class=\"stages\">\n"""
))

replacements.append((
"""    this.$('loss').addEventListener('input', () => {\n      this.$('lossValue').textContent = this.$('loss').value;\n      if (this.session) this.evaluate();\n    });\n    this.$('fail').addEventListener('click', () => this.failRandomNode());\n""",
"""    this.$('loss').addEventListener('input', () => {\n      this.$('lossValue').textContent = this.$('loss').value;\n      if (this.session) this.evaluate();\n    });\n    this.$('delay').addEventListener('input', () => {\n      const ms = Number(this.$('delay').value);\n      const label = ms === 0 ? 'sin pausa' : `${(ms / 1000).toFixed(1)} s`;\n      this.$('delayValue').textContent = label;\n      this.$('guideTiming').textContent = ms === 0 ? 'sin pausa' : `${label} / etapa`;\n    });\n    this.$('fail').addEventListener('click', () => this.failRandomNode());\n"""
))

replacements.append((
"""    this.$('rerun').addEventListener('click', () => this.newSession(false));\n    this.$('clear').addEventListener('click', () => {\n""",
"""    this.$('rerun').addEventListener('click', () => this.newSession(false));\n    this.$('replay').addEventListener('click', () => this.playCurrent());\n    this.$('clear').addEventListener('click', () => {\n"""
))

replacements.append((
"""      this.session = null;\n      this.baseResult = null;\n    });\n""",
"""      this.session = null;\n      this.baseResult = null;\n      this.currentResult = null;\n      this.$('replay').disabled = true;\n      this.setGuide(0, 'Qué hace Sigil', 'Escribí con el teclado gráfico y tocá ENVIAR. La demo mostrará cada transformación con la pausa que elijas.');\n    });\n"""
))

replacements.append((
"""  clearOutput() {\n    ['symbolStatus','cryptoStatus','shardStatus','nodeStatus','reconstructStatus'].forEach((id) => { this.$(id).textContent = 'esperando'; });\n    this.$('symbolPreview').textContent = '—'; this.$('wirePreview').textContent = '—'; this.$('reconstructPreview').textContent = '—';\n    this.$('shards').replaceChildren(); this.$('nodeList').replaceChildren(); this.renderBytes(this.$('receiverGlyphs'), []);\n    this.$('receiverState').textContent = 'esperando reconstrucción autenticada';\n  }\n\n  async newSession(live) {\n""",
"""  clearOutput() {\n    ['symbolStatus','cryptoStatus','shardStatus','nodeStatus','reconstructStatus'].forEach((id) => { this.$(id).textContent = 'esperando'; });\n    this.$('symbolPreview').textContent = '—'; this.$('wirePreview').textContent = '—'; this.$('reconstructPreview').textContent = '—';\n    this.$('shards').replaceChildren(); this.$('nodeList').replaceChildren(); this.renderBytes(this.$('receiverGlyphs'), []);\n    this.$('receiverState').textContent = 'esperando reconstrucción autenticada';\n  }\n\n  setGuide(step, title, text) {\n    this.$('guideStep').textContent = step ? `PASO ${step} / 5` : 'RECORRIDO EN VIVO';\n    this.$('guideTitle').textContent = title;\n    this.$('guideText').textContent = text;\n    this.$('guideBar').style.width = `${step ? step * 20 : 0}%`;\n  }\n\n  wait(ms) { return new Promise((resolve) => setTimeout(resolve, ms)); }\n\n  guidedDelay() { return Number(this.$('delay').value); }\n\n  async newSession(live) {\n"""
))

replacements.append((
"""    this.baseResult = JSON.parse(this.session.run(''));\n    this.previousDigest = previous;\n    this.rebuildTopology();\n    await this.animateStages(live);\n    this.evaluate();\n  }\n""",
"""    this.baseResult = JSON.parse(this.session.run(''));\n    this.previousDigest = previous;\n    this.rebuildTopology(false);\n    this.$('replay').disabled = false;\n    if (live) {\n      this.evaluate();\n      this.setGuide(0, 'Actualización en vivo', 'Mientras componés, Sigil actualiza el pipeline rápidamente. Tocá ENVIAR para verlo etapa por etapa.');\n      return;\n    }\n    await this.playCurrent();\n  }\n"""
))

replacements.append((
"""  rebuildTopology() {\n    if (!this.baseResult) return;\n    const count = Number(this.$('nodes').value);\n    this.failedNodes = new Set([...this.failedNodes].filter((index) => index < count));\n    this.assignments = this.baseResult.fragments.map((fragment, index) => ({ fragment, index, nodeIndex: this.stableHash(`${fragment.capability}:${index}`) % count }));\n    this.evaluate();\n  }\n""",
"""  rebuildTopology(render = true) {\n    if (!this.baseResult) return;\n    const count = Number(this.$('nodes').value);\n    this.failedNodes = new Set([...this.failedNodes].filter((index) => index < count));\n    this.assignments = this.baseResult.fragments.map((fragment, index) => ({ fragment, index, nodeIndex: this.stableHash(`${fragment.capability}:${index}`) % count }));\n    if (render) this.evaluate();\n  }\n"""
))

old_tail = """  async animateStages(live) {\n    const ids = ['stageSymbols','stageCrypto','stageShards','stageNodes','stageReconstruct'];\n    const delay = live ? 45 : 110;\n    for (const id of ids) {\n      const el = this.$(id); el.classList.add('flash'); await new Promise((r) => setTimeout(r, delay)); el.classList.remove('flash');\n    }\n  }\n"""
new_tail = """  async playCurrent() {\n    if (!this.session || !this.baseResult) return;\n    const token = ++this.playback;\n    const result = JSON.parse(this.session.run(this.missingSlots().join(',')));\n    this.currentResult = result;\n    this.$('replay').disabled = true;\n    this.clearOutput();\n    const delay = this.guidedDelay();\n\n    const step = async (number, id, title, text, reveal) => {\n      if (token !== this.playback) return false;\n      const el = this.$(id);\n      this.setGuide(number, title, text);\n      el.classList.add('flash');\n      reveal();\n      if (delay) await this.wait(delay);\n      el.classList.remove('flash');\n      return token === this.playback;\n    };\n\n    if (!await step(1, 'stageSymbols', 'El texto deja de ser texto del sistema', 'Las pulsaciones del Secure Canvas ya son bytes y SymbolId internos. La representación efímera cambia entre envelopes.', () => {\n      this.$('core').textContent = `v${result.version}`;\n      this.$('symbolStatus').textContent = `${result.symbol_count} SymbolId · mapa efímero`;\n      this.$('symbolPreview').textContent = result.symbol_codes.slice(0, 3).map((v) => v.slice(0, 14)).join(' · ') || '—';\n    })) return;\n\n    if (!await step(2, 'stageCrypto', 'Se cifra y vuelve a envolver', 'El stream binario entra en una capa AEAD y luego en otra capa AEAD independiente para transporte. Lo visible en red ya es ciphertext autenticado.', () => {\n      this.$('envelope').textContent = `${result.outer_wire_bytes} B`;\n      this.$('cryptoStatus').textContent = 'inner AEAD ✓ · outer AEAD ✓';\n      this.$('wirePreview').textContent = `${result.outer_wire_digest.slice(0, 24)}…`;\n    })) return;\n\n    if (!await step(3, 'stageShards', 'El ciphertext se convierte en un rompecabezas', 'Reed–Solomon divide el ciphertext externo en 20 shards. El receptor necesita un umbral suficiente, no necesariamente todas las piezas.', () => {\n      this.$('threshold').textContent = `${result.fragments_available}/${result.fragments_required} avail`;\n      this.$('shardStatus').textContent = `${result.fragments_total} shards · ${result.fragments_lost} no disponibles`;\n      this.renderShards(result);\n    })) return;\n\n    if (!await step(4, 'stageNodes', 'Las piezas se dispersan', 'La topología dibujada es simulada. Cada shard mostrado sí viene del core real; apagar un nodo quita sus shards del intento real de reconstrucción.', () => {\n      const usedNodes = new Set(this.assignments.map((a) => a.nodeIndex));\n      this.$('network').textContent = `${this.$('nodes').value} pool · ${usedNodes.size} usados`;\n      this.renderNodes(result);\n    })) return;\n\n    if (!await step(5, 'stageReconstruct', 'El receptor reconstruye antes de mostrar', 'Si quedan suficientes shards, el core recompone el ciphertext, verifica autenticidad, descifra las dos capas y recién entonces entrega símbolos al renderer.', () => {\n      this.$('reconstructStatus').textContent = 'verificando umbral y autenticidad…';\n    })) return;\n\n    this.renderResult(result);\n    const ok = result.reconstruction_possible && result.reconstruction_matches && result.receiver_matches;\n    this.setGuide(5, ok ? 'Mensaje reconstruido y autenticado' : 'Reconstrucción bloqueada', ok\n      ? 'El receptor obtuvo suficientes piezas, verificó el ciphertext y renderizó el contenido. Las claves no se muestran en la interfaz.'\n      : 'No quedaron suficientes piezas para alcanzar el umbral. Sigil no intenta mostrar un mensaje parcial o no autenticado.');\n    this.$('replay').disabled = false;\n  }\n"""
replacements.append((old_tail, new_tail))

for old, new in replacements:
    if old not in text:
        raise SystemExit('Patch target not found:\n' + old[:180])
    text = text.replace(old, new, 1)

path.write_text(text, encoding='utf-8')
