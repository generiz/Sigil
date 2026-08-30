from pathlib import Path

path = Path('web-demo/embed.js')
text = path.read_text(encoding='utf-8')

old = """  async newSession(live) {\n    if (!this.ready || !this.message.length) return;\n"""
new = """  async newSession(live) {\n    if (!this.ready || !this.message.length) return;\n    this.playback += 1;\n"""
if old not in text:
    raise SystemExit('newSession patch target not found')
text = text.replace(old, new, 1)

old = """    this.$('replay').disabled = true;\n    this.clearOutput();\n    const delay = this.guidedDelay();\n"""
new = """    this.$('replay').disabled = true;\n    this.clearOutput();\n    ['core','envelope','threshold','network'].forEach((id) => { this.$(id).textContent = '—'; });\n    this.$('stageReconstruct').classList.remove('fail');\n    const delay = this.guidedDelay();\n"""
if old not in text:
    raise SystemExit('playCurrent patch target not found')
text = text.replace(old, new, 1)

path.write_text(text, encoding='utf-8')
