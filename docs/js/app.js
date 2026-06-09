// ============================================
// App — Belajar Rust Interactive
// ============================================

let monacoEditor = null;
let currentLesson = null;
let activeTab     = 'materi';
let progress      = loadProgress();

// ============ INIT ============
window.addEventListener('DOMContentLoaded', () => {
  buildSidebar();
  updateProgressBar();
  initMonaco();

  // Buka lesson pertama
  const firstModule = CURRICULUM[0];
  const firstLesson = firstModule.lessons[0];
  openLesson(firstModule.id, firstLesson.id);
});

// ============ MONACO INIT ============
function initMonaco() {
  require.config({
    paths: { vs: 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.44.0/min/vs' }
  });
  require(['vs/editor/editor.main'], () => {
    monacoEditor = monaco.editor.create(document.getElementById('editor-container'), {
      value: '// Pilih lesson dari sidebar',
      language: 'rust',
      theme: 'vs-dark',
      fontSize: 14,
      fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
      fontLigatures: true,
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      lineNumbers: 'on',
      renderLineHighlight: 'line',
      automaticLayout: true,
      tabSize: 4,
      wordWrap: 'on',
      padding: { top: 12, bottom: 12 },
    });
  });
}

// ============ SIDEBAR ============
function buildSidebar() {
  const nav = document.getElementById('sidebar-nav');
  nav.innerHTML = '';

  CURRICULUM.forEach((module, mi) => {
    const wrap = document.createElement('div');
    wrap.className = 'module-wrap';

    const header = document.createElement('div');
    header.className = 'module-header open';
    header.innerHTML = `
      <div class="module-title">
        <span class="module-icon">${module.icon}</span>
        <span>${module.title}</span>
      </div>
      <span class="module-chevron">▶</span>
    `;
    header.addEventListener('click', () => toggleModule(header, lessons));

    const lessons = document.createElement('div');
    lessons.className = 'module-lessons open';

    module.lessons.forEach(lesson => {
      const item = document.createElement('div');
      item.className = 'lesson-item';
      item.dataset.moduleId = module.id;
      item.dataset.lessonId = lesson.id;

      const isDone = progress[`${module.id}:${lesson.id}`];
      if (isDone) item.classList.add('done');

      item.innerHTML = `<span class="lesson-dot"></span>${lesson.title}`;
      item.addEventListener('click', () => openLesson(module.id, lesson.id));
      lessons.appendChild(item);
    });

    wrap.appendChild(header);
    wrap.appendChild(lessons);
    nav.appendChild(wrap);
  });
}

function toggleModule(header, lessons) {
  header.classList.toggle('open');
  lessons.classList.toggle('open');
}

// ============ SEARCH ============
document.getElementById('sidebar-search').addEventListener('input', (e) => {
  const q = e.target.value.toLowerCase();
  document.querySelectorAll('.lesson-item').forEach(item => {
    const title = item.textContent.toLowerCase();
    item.style.display = title.includes(q) ? '' : 'none';
  });
});

// ============ OPEN LESSON ============
function openLesson(moduleId, lessonId) {
  const module = CURRICULUM.find(m => m.id === moduleId);
  if (!module) return;
  const lesson = module.lessons.find(l => l.id === lessonId);
  if (!lesson) return;

  currentLesson = { moduleId, lessonId, lesson };

  // Update sidebar active state
  document.querySelectorAll('.lesson-item').forEach(item => {
    item.classList.remove('active');
    if (item.dataset.moduleId === moduleId && item.dataset.lessonId === lessonId) {
      item.classList.add('active');
    }
  });

  // Update title
  document.getElementById('content-title').textContent = `${module.title} › ${lesson.title}`;

  // Render materi
  renderMateri(lesson);

  // Set exercise
  renderExercise(lesson);

  // Set editor code based on active tab
  if (activeTab === 'materi') {
    setEditorCode(lesson.defaultCode);
  } else {
    setEditorCode(lesson.exercise.starterCode);
  }

  // Clear output
  clearOutput();

  // Switch to materi tab
  switchTab('materi');
}

// ============ RENDER MATERI ============
function renderMateri(lesson) {
  const container = document.getElementById('materi-content');

  // Convert markdown-like content to HTML
  let html = lesson.content
    .trim()
    // Headers
    .replace(/^### (.+)$/gm, '<h3>$1</h3>')
    .replace(/^## (.+)$/gm, '<h2>$1</h2>')
    .replace(/^# (.+)$/gm, '<h1>$1</h1>')
    // Bold
    .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
    // Inline code
    .replace(/`([^`\n]+)`/g, '<code>$1</code>')
    // Code blocks — pakai highlight.js
    .replace(/```(\w*)\n([\s\S]*?)```/g, (_, lang, code) => {
      const trimmed  = code.trim();
      const language = lang || 'rust';
      const highlighted = (typeof hljs !== 'undefined')
        ? hljs.highlight(trimmed, { language, ignoreIllegals: true }).value
        : trimmed.replace(/</g, '&lt;').replace(/>/g, '&gt;');
      return `<pre class="code-block" data-code="${encodeURIComponent(trimmed)}"><button class="btn-copy-code" onclick="copyCode(this)">Copy</button><code class="hljs language-${language}">${highlighted}</code></pre>`;
    })
    // Lists
    .replace(/^- (.+)$/gm, '<li>$1</li>')
    .replace(/(<li>.*<\/li>\n?)+/g, '<ul>$&</ul>')
    // Paragraphs
    .replace(/\n\n(?!<)/g, '</p><p>')
    // concept-box passthrough (already HTML)
    ;

  container.innerHTML = `<div class="materi-content">${html}</div>`;

  // Add "Coba di Editor" buttons after code blocks
  container.querySelectorAll('.code-block').forEach(block => {
    const code = decodeURIComponent(block.dataset.code || '');
    if (code && code.includes('fn main')) {
      const btn = document.createElement('button');
      btn.className = 'try-btn';
      btn.innerHTML = '▶ Coba di Editor';
      btn.onclick = () => {
        setEditorCode(code);
        switchTab('materi');
        showToast('Kode dipindahkan ke editor!', 'info');
      };
      block.after(btn);
    }
  });

  // Nav buttons
  const navDiv = document.createElement('div');
  navDiv.className = 'nav-buttons';

  const prev = getPrevLesson();
  const next = getNextLesson();

  const prevBtn = document.createElement('button');
  prevBtn.className = `nav-btn ${prev ? '' : 'disabled'}`;
  prevBtn.innerHTML = '← Sebelumnya';
  if (prev) prevBtn.onclick = () => openLesson(prev.moduleId, prev.lessonId);

  const nextBtn = document.createElement('button');
  nextBtn.className = `nav-btn primary ${next ? '' : 'disabled'}`;
  nextBtn.innerHTML = 'Selanjutnya →';
  if (next) nextBtn.onclick = () => openLesson(next.moduleId, next.lessonId);

  navDiv.appendChild(prevBtn);
  navDiv.appendChild(nextBtn);
  container.appendChild(navDiv);
}


function copyCode(btn) {
  const code = decodeURIComponent(btn.parentElement.dataset.code || '');
  navigator.clipboard.writeText(code).then(() => {
    btn.textContent = 'Copied!';
    setTimeout(() => btn.textContent = 'Copy', 2000);
  });
}

// ============ RENDER EXERCISE ============
function renderExercise(lesson) {
  const ex = lesson.exercise;
  const desc = document.getElementById('exercise-desc');

  let tasksHtml = ex.tasks.map((t, i) =>
    `<li><strong>${i+1}.</strong> ${t}</li>`
  ).join('');

  let hintsHtml = ex.hints.map(h => `<p>💡 ${h}</p>`).join('');

  desc.innerHTML = `
    <h3>🎯 ${ex.title}</h3>
    <p>${ex.desc}</p>
    <ul>${tasksHtml}</ul>
    <button class="hint-toggle" onclick="toggleHints(this)">💡 Tampilkan Hints</button>
    <div class="hints-box">${hintsHtml}</div>
  `;
}

function toggleHints(btn) {
  const box = btn.nextElementSibling;
  box.classList.toggle('show');
  btn.textContent = box.classList.contains('show') ? '🙈 Sembunyikan Hints' : '💡 Tampilkan Hints';
}

// ============ TABS ============
function switchTab(tab) {
  activeTab = tab;

  document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
  document.querySelectorAll('[data-tab]').forEach(t => {
    if (t.dataset.tab === tab) t.classList.add('active');
  });

  document.querySelectorAll('.panel-materi, .panel-exercise').forEach(p => {
    p.classList.remove('active');
  });

  if (tab === 'materi') {
    document.querySelector('.panel-materi').classList.add('active');
    if (currentLesson) setEditorCode(currentLesson.lesson.defaultCode);
  } else {
    document.querySelector('.panel-exercise').classList.add('active');
    if (currentLesson) setEditorCode(currentLesson.lesson.exercise.starterCode);
  }

  clearOutput();
}

// Bind tab clicks
document.querySelectorAll('.tab').forEach(tab => {
  tab.addEventListener('click', () => switchTab(tab.dataset.tab));
});

// ============ EDITOR ============
function setEditorCode(code) {
  if (monacoEditor && code) {
    monacoEditor.setValue(code);
    monacoEditor.revealLine(1);
  }
}

document.getElementById('btn-reset-code').addEventListener('click', () => {
  if (!currentLesson) return;
  const code = activeTab === 'materi'
    ? currentLesson.lesson.defaultCode
    : currentLesson.lesson.exercise.starterCode;
  setEditorCode(code);
  showToast('Kode direset!', 'info');
});

// ============ RUN CODE ============
document.getElementById('btn-run').addEventListener('click', runCode);

async function runCode() {
  if (!monacoEditor) return;
  const code = monacoEditor.getValue();
  if (!code.trim()) return;

  const btn = document.getElementById('btn-run');
  btn.disabled = true;
  btn.classList.add('running');
  btn.innerHTML = '<span class="spinner"></span> Running...';

  clearOutput();
  showOutput('⏳ Mengirim ke Rust Playground...', 'stdout');

  const startTime = Date.now();

  try {
    const response = await fetch('https://play.rust-lang.org/execute', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        channel:   'stable',
        edition:   '2021',
        mode:      'debug',
        crateType: 'bin',
        tests:     false,
        code:      code,
      }),
    });

    if (!response.ok) throw new Error(`HTTP ${response.status}`);

    const result = await response.json();
    const elapsed = Date.now() - startTime;

    clearOutput();
    document.getElementById('output-time').textContent = `${elapsed}ms`;

    if (result.success) {
      document.getElementById('output-status').className = 'output-status success';
      document.getElementById('output-status').textContent = '✓ Sukses';
      showOutput(result.stdout || '(tidak ada output)', 'stdout');
      if (result.stderr) showOutput('\n' + result.stderr, 'stderr');
    } else {
      document.getElementById('output-status').className = 'output-status error';
      document.getElementById('output-status').textContent = '✗ Error';
      if (result.stderr) showOutput(result.stderr, 'stderr');
      if (result.stdout) showOutput(result.stdout, 'stdout');
    }

  } catch (err) {
    clearOutput();
    document.getElementById('output-status').className = 'output-status error';
    document.getElementById('output-status').textContent = '✗ Network Error';
    showOutput(`❌ Gagal terhubung ke Rust Playground:\n${err.message}\n\nPastikan internet tersambung.`, 'stderr');
  } finally {
    btn.disabled = false;
    btn.classList.remove('running');
    btn.innerHTML = '▶ Jalankan';
  }
}

function showOutput(text, type) {
  const content = document.getElementById('output-content');
  const span = document.createElement('span');
  span.className = `output-${type}`;
  span.textContent = text;
  content.appendChild(span);
  content.scrollTop = content.scrollHeight;
}

function clearOutput() {
  const content = document.getElementById('output-content');
  content.innerHTML = '<span class="output-placeholder">// Output akan muncul di sini setelah kode dijalankan</span>';
  document.getElementById('output-status').className = 'output-status';
  document.getElementById('output-time').textContent = '';
}

// ============ MARK DONE ============
document.getElementById('btn-mark-done').addEventListener('click', () => {
  if (!currentLesson) return;
  const key = `${currentLesson.moduleId}:${currentLesson.lessonId}`;
  progress[key] = true;
  saveProgress();
  updateProgressBar();
  buildSidebar();

  // Re-select active lesson
  document.querySelectorAll('.lesson-item').forEach(item => {
    if (item.dataset.moduleId === currentLesson.moduleId &&
        item.dataset.lessonId === currentLesson.lessonId) {
      item.classList.add('active');
    }
  });

  showToast('✅ Lesson ditandai selesai!', 'success');
});

// ============ PROGRESS ============
function loadProgress() {
  try { return JSON.parse(localStorage.getItem('belajar-rust-progress') || '{}'); }
  catch { return {}; }
}

function saveProgress() {
  localStorage.setItem('belajar-rust-progress', JSON.stringify(progress));
}

function updateProgressBar() {
  const total = CURRICULUM.reduce((sum, m) => sum + m.lessons.length, 0);
  const done  = Object.keys(progress).filter(k => progress[k]).length;
  const pct   = total > 0 ? Math.round((done / total) * 100) : 0;

  document.getElementById('progress-fill').style.width = `${pct}%`;
  document.getElementById('progress-text').textContent = `${done}/${total} lesson`;
}

document.getElementById('btn-reset-progress').addEventListener('click', () => {
  if (!confirm('Reset semua progress? Ini tidak bisa dibatalkan.')) return;
  progress = {};
  saveProgress();
  updateProgressBar();
  buildSidebar();
  if (currentLesson) {
    document.querySelectorAll('.lesson-item').forEach(item => {
      if (item.dataset.moduleId === currentLesson.moduleId &&
          item.dataset.lessonId === currentLesson.lessonId) {
        item.classList.add('active');
      }
    });
  }
  showToast('Progress direset!', 'info');
});

// ============ TOAST ============
function showToast(msg, type = 'info') {
  const toast = document.getElementById('toast');
  toast.textContent = msg;
  toast.className = `toast show ${type}`;
  setTimeout(() => toast.classList.remove('show'), 3000);
}

// ============ NAV HELPERS ============
function getAllLessons() {
  const all = [];
  CURRICULUM.forEach(m => m.lessons.forEach(l => all.push({ moduleId: m.id, lessonId: l.id })));
  return all;
}

function getCurrentIndex() {
  if (!currentLesson) return -1;
  const all = getAllLessons();
  return all.findIndex(l => l.moduleId === currentLesson.moduleId && l.lessonId === currentLesson.lessonId);
}

function getPrevLesson() {
  const idx = getCurrentIndex();
  if (idx <= 0) return null;
  return getAllLessons()[idx - 1];
}

function getNextLesson() {
  const all = getAllLessons();
  const idx = getCurrentIndex();
  if (idx < 0 || idx >= all.length - 1) return null;
  return all[idx + 1];
}

// Keyboard shortcuts
document.addEventListener('keydown', (e) => {
  if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
    e.preventDefault();
    runCode();
  }
});
