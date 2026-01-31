pub const EDIT_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}Edit {{ record_id }} - {{ site.title }}{% endblock %}

{% block head %}
<style>
    .edit-container {
        display: flex;
        flex-direction: column;
        height: calc(100vh - 140px);
        min-height: 500px;
    }
    .editor-layout {
        display: grid;
        grid-template-columns: 1fr 8px 1fr;
        gap: 0;
        flex: 1;
        min-height: 0;
    }
    .editor-layout.editor-only { grid-template-columns: 1fr; }
    .editor-layout.editor-only .preview-pane,
    .editor-layout.editor-only .editor-gutter { display: none; }
    .editor-layout.preview-only { grid-template-columns: 1fr; }
    .editor-layout.preview-only .editor-pane-wrapper,
    .editor-layout.preview-only .editor-gutter { display: none; }
    .editor-gutter {
        background: linear-gradient(to right, #1e293b, #334155, #1e293b);
        border-radius: 4px;
        margin: 1rem 0;
    }
    .editor-toolbar {
        position: sticky;
        top: 0;
        z-index: 10;
    }
    .editor-pane-wrapper, .preview-pane {
        display: flex;
        flex-direction: column;
        overflow: hidden;
        min-height: 0;
    }
    .editor-pane-wrapper textarea {
        flex: 1;
        resize: none;
        font-family: 'JetBrains Mono', monospace;
        font-size: 14px;
        line-height: 1.6;
        tab-size: 2;
        min-height: 0;
    }
    .preview-content {
        flex: 1;
        overflow-y: auto;
        min-height: 0;
    }
    @media (max-width: 768px) {
        .editor-layout { grid-template-columns: 1fr; grid-template-rows: 1fr 8px 1fr; }
        .editor-layout.editor-only, .editor-layout.preview-only { grid-template-rows: 1fr; }
        .editor-gutter {
            margin: 0 1rem;
            background: linear-gradient(to bottom, #1e293b, #334155, #1e293b);
        }
    }
    .field-input {
        background: #1e293b;
        border: 1px solid #334155;
        border-radius: 0.5rem;
        padding: 0.5rem 0.75rem;
        color: #e2e8f0;
        font-size: 0.875rem;
        width: 100%;
        transition: border-color 0.2s, box-shadow 0.2s;
    }
    .field-input:focus, .field-input:focus-within {
        outline: none;
        border-color: var(--primary);
        box-shadow: 0 0 0 2px rgba(0, 124, 67, 0.2);
    }
    .field-input::placeholder { color: #64748b; }
    select.field-input { cursor: pointer; }
    .tag-input {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
        padding: 0.5rem;
        min-height: 42px;
        align-items: center;
        cursor: text;
    }
    .tag {
        display: inline-flex;
        align-items: center;
        gap: 0.25rem;
        background: #334155;
        padding: 0.25rem 0.5rem;
        border-radius: 0.375rem;
        font-size: 0.75rem;
        color: #94a3b8;
    }
    .tag button {
        color: #64748b;
        font-size: 1rem;
        line-height: 1;
        background: none;
        border: none;
        cursor: pointer;
        padding: 0;
    }
    .tag button:hover { color: #ef4444; }
    .tag button:focus-visible { outline: 2px solid var(--accent); outline-offset: 1px; }
    .view-btn { padding: 0.375rem; border-radius: 0.375rem; color: #64748b; transition: all 0.15s; }
    .view-btn:hover { background: #334155; color: #e2e8f0; }
    .view-btn.active { background: var(--primary); color: white; }
    .view-btn:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
    #saveBtn:focus-visible, .cancel-btn:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
    .toolbar-btn {
        padding: 0.375rem;
        border-radius: 0.25rem;
        color: #94a3b8;
        transition: all 0.15s;
        display: flex;
        align-items: center;
        gap: 0.125rem;
    }
    .toolbar-btn:hover { background: #334155; color: #e2e8f0; }
    .toolbar-btn:active { background: var(--primary); color: white; }
    .toolbar-btn:focus-visible { outline: 2px solid var(--accent); outline-offset: 1px; }
</style>
{% endblock %}

{% block content %}
<div class="edit-container">
    <!-- Header -->
    <div class="flex justify-between items-center mb-4 flex-shrink-0">
        <div class="flex items-center gap-4">
            <a href="/records/{{ record_id }}" class="p-2 hover:bg-slate-700 rounded-lg transition-colors text-slate-400 hover:text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-piper-accent" title="Back">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path></svg>
            </a>
            <div>
                <h1 class="text-2xl font-bold text-white">Edit <span id="displayId">{{ record_id }}</span></h1>
                <p class="text-slate-400 text-sm" id="displayTitle">{{ record_title }}</p>
            </div>
        </div>
        <div class="flex gap-3 items-center">
            <a href="/records/{{ record_id }}" class="btn btn-ghost">Cancel</a>
            <button id="saveBtn" class="btn btn-primary">
                <svg id="saveIcon" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                <span id="saveBtnText">Save</span>
            </button>
        </div>
    </div>

    <!-- Status bar -->
    <div id="statusBar" class="mb-4 px-4 py-2 rounded-lg text-sm hidden flex-shrink-0"></div>

    <!-- Metadata Section -->
    <div class="bg-piper-card border border-slate-700 rounded-xl mb-4 overflow-hidden flex-shrink-0">
        <button type="button" id="metadataToggle" aria-expanded="true" aria-controls="metadataFields" class="w-full px-4 py-3 bg-slate-800/50 border-b border-slate-700 flex justify-between items-center cursor-pointer hover:bg-slate-800 transition-colors focus-visible:outline focus-visible:outline-2 focus-visible:outline-piper-accent focus-visible:outline-offset-[-2px]" onclick="toggleMetadata()">
            <span class="text-xs font-mono uppercase tracking-wider text-slate-500">Metadata</span>
            <svg id="metadataChevron" class="w-4 h-4 text-slate-500 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
        </button>
        <div id="metadataFields" class="px-4 py-3 grid grid-cols-1 md:grid-cols-2 gap-x-4 gap-y-3">
            <div>
                <label for="fieldTitle" class="label text-xs opacity-60">Title</label>
                <input type="text" id="fieldTitle" class="input input-bordered w-full" placeholder="Record title">
            </div>
            <div class="flex gap-4 items-end">
                <div class="flex-1">
                    <label for="fieldStatus" class="label text-xs opacity-60">Status</label>
                    <select id="fieldStatus" class="select select-bordered w-full">
                        <option value="proposed">Proposed</option>
                        <option value="draft">Draft</option>
                        <option value="accepted">Accepted</option>
                        <option value="active">Active</option>
                        <option value="deprecated">Deprecated</option>
                        <option value="superseded">Superseded</option>
                        <option value="open">Open</option>
                        <option value="resolved">Resolved</option>
                        <option value="cancelled">Cancelled</option>
                    </select>
                </div>
                <label for="fieldFoundational" class="flex items-center gap-2 cursor-pointer select-none h-[42px] whitespace-nowrap">
                    <input type="checkbox" id="fieldFoundational" class="checkbox checkbox-primary">
                    <span class="label-text">Core</span>
                </label>
            </div>
            <div>
                <label for="authorInput" class="block text-xs text-slate-500 mb-1">Authors</label>
                <div id="authorsContainer" class="field-input tag-input" onclick="document.getElementById('authorInput').focus()">
                    <input type="text" id="authorInput" class="bg-transparent border-none outline-none text-sm flex-1 min-w-[80px]" placeholder="Add author...">
                </div>
            </div>
            <div>
                <label for="tagInput" class="block text-xs text-slate-500 mb-1">Tags</label>
                <div id="tagsContainer" class="field-input tag-input" onclick="document.getElementById('tagInput').focus()">
                    <input type="text" id="tagInput" class="bg-transparent border-none outline-none text-sm flex-1 min-w-[80px]" placeholder="Add tag...">
                </div>
            </div>
        </div>
    </div>

    <!-- Editor -->
    <div id="editorLayout" class="editor-layout">
        <div class="editor-pane-wrapper bg-piper-card border border-slate-700 rounded-xl overflow-hidden mr-1">
            <div class="px-4 py-2 bg-slate-800/50 border-b border-slate-700 flex justify-between items-center flex-shrink-0">
                <span class="text-xs font-mono uppercase tracking-wider text-slate-500">Content</span>
                <span id="cursorPos" class="text-xs text-slate-500 font-mono">Ln 1, Col 1</span>
            </div>
            <!-- Formatting Toolbar -->
            <div class="editor-toolbar px-2 py-1.5 bg-slate-800/80 backdrop-blur-sm border-b border-slate-700/50 flex flex-wrap gap-1 flex-shrink-0">
                <div class="flex gap-0.5 border-r border-slate-700 pr-2 mr-1">
                    <button type="button" class="toolbar-btn" onclick="insertFormat('heading')" title="Heading (Ctrl+H)">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"/></svg>
                        <span class="text-xs">H</span>
                    </button>
                </div>
                <div class="flex gap-0.5 border-r border-slate-700 pr-2 mr-1">
                    <button type="button" class="toolbar-btn" onclick="insertFormat('bold')" title="Bold (Ctrl+B)">
                        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor"><path d="M15.6 10.79c.97-.67 1.65-1.77 1.65-2.79 0-2.26-1.75-4-4-4H7v14h7.04c2.09 0 3.71-1.7 3.71-3.79 0-1.52-.86-2.82-2.15-3.42zM10 6.5h3c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5h-3v-3zm3.5 9H10v-3h3.5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5z"/></svg>
                    </button>
                    <button type="button" class="toolbar-btn" onclick="insertFormat('italic')" title="Italic (Ctrl+I)">
                        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor"><path d="M10 4v3h2.21l-3.42 8H6v3h8v-3h-2.21l3.42-8H18V4z"/></svg>
                    </button>
                    <button type="button" class="toolbar-btn" onclick="insertFormat('strikethrough')" title="Strikethrough">
                        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor"><path d="M10 19h4v-3h-4v3zM5 4v3h5v3h4V7h5V4H5zM3 14h18v-2H3v2z"/></svg>
                    </button>
                    <button type="button" class="toolbar-btn" onclick="insertFormat('code')" title="Inline Code">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"/></svg>
                    </button>
                </div>
                <div class="flex gap-0.5 border-r border-slate-700 pr-2 mr-1">
                    <button type="button" class="toolbar-btn" onclick="insertFormat('link')" title="Link (Ctrl+K)">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"/></svg>
                    </button>
                    <button type="button" class="toolbar-btn" onclick="insertFormat('image')" title="Image">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/></svg>
                    </button>
                </div>
                <div class="flex gap-0.5 border-r border-slate-700 pr-2 mr-1">
                    <button type="button" class="toolbar-btn" onclick="insertFormat('ul')" title="Bullet List">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/></svg>
                    </button>
                    <button type="button" class="toolbar-btn" onclick="insertFormat('ol')" title="Numbered List">
                        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor"><path d="M2 17h2v.5H3v1h1v.5H2v1h3v-4H2v1zm1-9h1V4H2v1h1v3zm-1 3h1.8L2 13.1v.9h3v-1H3.2L5 10.9V10H2v1zm5-6v2h14V5H7zm0 14h14v-2H7v2zm0-6h14v-2H7v2z"/></svg>
                    </button>
                    <button type="button" class="toolbar-btn" onclick="insertFormat('task')" title="Task List">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"/></svg>
                    </button>
                </div>
                <div class="flex gap-0.5 border-r border-slate-700 pr-2 mr-1">
                    <button type="button" class="toolbar-btn" onclick="insertFormat('quote')" title="Quote">
                        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor"><path d="M6 17h3l2-4V7H5v6h3zm8 0h3l2-4V7h-6v6h3z"/></svg>
                    </button>
                    <button type="button" class="toolbar-btn" onclick="insertFormat('codeblock')" title="Code Block">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/></svg>
                    </button>
                    <button type="button" class="toolbar-btn" onclick="insertFormat('hr')" title="Horizontal Rule">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4"/></svg>
                    </button>
                </div>
                <div class="flex gap-0.5">
                    <button type="button" class="toolbar-btn" onclick="insertFormat('table')" title="Table">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M3 14h18m-9-4v8m-7 0h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"/></svg>
                    </button>
                    <button type="button" class="toolbar-btn" onclick="insertFormat('mention')" title="Mention @user">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"/></svg>
                    </button>
                </div>
            </div>
            <textarea id="editor" class="w-full p-4 bg-transparent text-slate-200 border-none outline-none" spellcheck="false" placeholder="Write your content here..."></textarea>
        </div>
        <div class="editor-gutter"></div>
        <div class="preview-pane bg-piper-card border border-slate-700 rounded-xl overflow-hidden ml-1">
            <div class="px-4 py-2 bg-slate-800/50 border-b border-slate-700 flex justify-between items-center flex-shrink-0">
                <span class="text-xs font-mono uppercase tracking-wider text-slate-500">Preview</span>
                <div class="flex gap-1">
                    <button type="button" class="view-btn" data-view="split" title="Split View" onclick="setView('split')">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7"></path></svg>
                    </button>
                    <button type="button" class="view-btn" data-view="editor-only" title="Editor Only" onclick="setView('editor-only')">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path></svg>
                    </button>
                    <button type="button" class="view-btn" data-view="preview-only" title="Preview Only" onclick="setView('preview-only')">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path></svg>
                    </button>
                </div>
            </div>
            <div id="preview" class="preview-content p-4 text-slate-300 content"></div>
        </div>
    </div>

    <div class="mt-4 text-xs text-slate-500 flex gap-6 flex-shrink-0">
        <span><kbd class="px-1.5 py-0.5 bg-slate-800 rounded border border-slate-700">Ctrl</kbd>+<kbd class="px-1.5 py-0.5 bg-slate-800 rounded border border-slate-700">S</kbd> Save</span>
        <span><kbd class="px-1.5 py-0.5 bg-slate-800 rounded border border-slate-700">Esc</kbd> Cancel</span>
    </div>
</div>

<!-- Hidden field for raw content -->
<textarea id="rawContent" style="display:none">{{ raw_content }}</textarea>
{% endblock %}

{% block scripts %}
<script>
const recordId = '{{ record_id }}';
const editor = document.getElementById('editor');
const preview = document.getElementById('preview');
const saveBtn = document.getElementById('saveBtn');
const statusBar = document.getElementById('statusBar');
const cursorPos = document.getElementById('cursorPos');
const rawContent = document.getElementById('rawContent').value;

// Form fields
const fieldTitle = document.getElementById('fieldTitle');
const fieldStatus = document.getElementById('fieldStatus');
const fieldFoundational = document.getElementById('fieldFoundational');
const authorInput = document.getElementById('authorInput');
const tagInput = document.getElementById('tagInput');
const authorsContainer = document.getElementById('authorsContainer');
const tagsContainer = document.getElementById('tagsContainer');

let frontmatter = {};
let authors = [];
let tags = [];
let links = {};
let originalFull = rawContent;
let isDirty = false;

// Parse frontmatter from raw content
function parseFrontmatter(content) {
    if (!content.startsWith('---')) return { frontmatter: {}, body: content };
    const endIdx = content.indexOf('---', 3);
    if (endIdx === -1) return { frontmatter: {}, body: content };

    const yamlStr = content.substring(3, endIdx).trim();
    const body = content.substring(endIdx + 3).trim();

    // Simple YAML parser for our use case
    const fm = {};
    const lines = yamlStr.split('\n');
    let currentKey = null;
    let inArray = false;
    let arrayKey = null;
    let inLinks = false;

    for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed || trimmed.startsWith('#')) continue;

        if (line.startsWith('  ') && inLinks) {
            // Links sub-key
            const match = trimmed.match(/^(\w+):\s*\[(.*)\]$/);
            if (match) {
                if (!fm.links) fm.links = {};
                fm.links[match[1]] = match[2] ? match[2].split(',').map(s => s.trim()) : [];
            }
            continue;
        }

        inLinks = false;
        const colonIdx = trimmed.indexOf(':');
        if (colonIdx === -1) continue;

        const key = trimmed.substring(0, colonIdx).trim();
        let value = trimmed.substring(colonIdx + 1).trim();

        if (key === 'links') {
            inLinks = true;
            fm.links = {};
            continue;
        }

        // Handle arrays like [a, b, c]
        if (value.startsWith('[') && value.endsWith(']')) {
            const inner = value.slice(1, -1);
            fm[key] = inner ? inner.split(',').map(s => s.trim().replace(/^["']|["']$/g, '')) : [];
        } else if (value === 'true') {
            fm[key] = true;
        } else if (value === 'false') {
            fm[key] = false;
        } else {
            // Remove quotes
            fm[key] = value.replace(/^["']|["']$/g, '');
        }
    }

    return { frontmatter: fm, body };
}

// Initialize from raw content
function initFromRaw() {
    const { frontmatter: fm, body } = parseFrontmatter(rawContent);
    frontmatter = fm;

    fieldTitle.value = fm.title || '';
    fieldStatus.value = fm.status || 'proposed';
    fieldFoundational.checked = fm.core || false;

    authors = fm.authors || [];
    tags = fm.tags || [];
    links = fm.links || {};

    renderAuthors();
    renderTags();
    editor.value = body;
    updatePreview();
}

function renderAuthors() {
    // Clear existing tags (keep input)
    authorsContainer.querySelectorAll('.tag').forEach(t => t.remove());
    authors.forEach((a, i) => {
        const tag = document.createElement('span');
        tag.className = 'tag';
        tag.innerHTML = `${a}<button onclick="removeAuthor(${i})">&times;</button>`;
        authorsContainer.insertBefore(tag, authorInput);
    });
}

function renderTags() {
    tagsContainer.querySelectorAll('.tag').forEach(t => t.remove());
    tags.forEach((t, i) => {
        const tag = document.createElement('span');
        tag.className = 'tag';
        tag.innerHTML = `#${t}<button onclick="removeTag(${i})">&times;</button>`;
        tagsContainer.insertBefore(tag, tagInput);
    });
}

function removeAuthor(idx) {
    authors.splice(idx, 1);
    renderAuthors();
    markDirty();
}

function removeTag(idx) {
    tags.splice(idx, 1);
    renderTags();
    markDirty();
}

authorInput.addEventListener('keydown', (e) => {
    if (e.key === 'Enter' || e.key === ',') {
        e.preventDefault();
        const val = authorInput.value.trim().replace(',', '');
        if (val && !authors.includes(val)) {
            authors.push(val);
            renderAuthors();
            markDirty();
        }
        authorInput.value = '';
    }
    if (e.key === 'Backspace' && !authorInput.value && authors.length) {
        authors.pop();
        renderAuthors();
        markDirty();
    }
});

tagInput.addEventListener('keydown', (e) => {
    if (e.key === 'Enter' || e.key === ',') {
        e.preventDefault();
        const val = tagInput.value.trim().replace(',', '').replace('#', '');
        if (val && !tags.includes(val)) {
            tags.push(val);
            renderTags();
            markDirty();
        }
        tagInput.value = '';
    }
    if (e.key === 'Backspace' && !tagInput.value && tags.length) {
        tags.pop();
        renderTags();
        markDirty();
    }
});

function toggleMetadata() {
    const fields = document.getElementById('metadataFields');
    const chevron = document.getElementById('metadataChevron');
    const toggle = document.getElementById('metadataToggle');
    const isExpanded = !fields.classList.contains('hidden');
    fields.classList.toggle('hidden');
    chevron.style.transform = !isExpanded ? '' : 'rotate(-90deg)';
    toggle.setAttribute('aria-expanded', !isExpanded);
}

// Formatting toolbar
function insertFormat(type) {
    const start = editor.selectionStart;
    const end = editor.selectionEnd;
    const selected = editor.value.substring(start, end);
    const before = editor.value.substring(0, start);
    const after = editor.value.substring(end);

    let insert = '';
    let cursorOffset = 0;

    switch(type) {
        case 'bold':
            insert = `**${selected || 'bold text'}**`;
            cursorOffset = selected ? insert.length : 2;
            break;
        case 'italic':
            insert = `*${selected || 'italic text'}*`;
            cursorOffset = selected ? insert.length : 1;
            break;
        case 'strikethrough':
            insert = `~~${selected || 'strikethrough'}~~`;
            cursorOffset = selected ? insert.length : 2;
            break;
        case 'code':
            insert = `\`${selected || 'code'}\``;
            cursorOffset = selected ? insert.length : 1;
            break;
        case 'heading':
            const lineStart = before.lastIndexOf('\n') + 1;
            const prefix = before.substring(lineStart);
            if (prefix.startsWith('### ')) {
                editor.value = before.substring(0, lineStart) + prefix.substring(4) + selected + after;
                editor.selectionStart = editor.selectionEnd = start - 4;
            } else if (prefix.startsWith('## ')) {
                editor.value = before.substring(0, lineStart) + '### ' + prefix.substring(3) + selected + after;
                editor.selectionStart = editor.selectionEnd = start + 1;
            } else if (prefix.startsWith('# ')) {
                editor.value = before.substring(0, lineStart) + '## ' + prefix.substring(2) + selected + after;
                editor.selectionStart = editor.selectionEnd = start;
            } else {
                insert = '# ';
                cursorOffset = 2;
            }
            if (!insert) { updatePreview(); markDirty(); editor.focus(); return; }
            break;
        case 'link':
            insert = selected ? `[${selected}](url)` : '[link text](url)';
            cursorOffset = selected ? insert.length - 4 : 1;
            break;
        case 'image':
            insert = selected ? `![${selected}](image-url)` : '![alt text](image-url)';
            cursorOffset = selected ? insert.length - 10 : 2;
            break;
        case 'ul':
            insert = (before.endsWith('\n') || !before) ? '- ' : '\n- ';
            cursorOffset = insert.length;
            break;
        case 'ol':
            insert = (before.endsWith('\n') || !before) ? '1. ' : '\n1. ';
            cursorOffset = insert.length;
            break;
        case 'task':
            insert = (before.endsWith('\n') || !before) ? '- [ ] ' : '\n- [ ] ';
            cursorOffset = insert.length;
            break;
        case 'quote':
            insert = (before.endsWith('\n') || !before) ? '> ' : '\n> ';
            cursorOffset = insert.length;
            break;
        case 'codeblock':
            insert = (before.endsWith('\n') || !before) ? '```\n' + (selected || 'code') + '\n```' : '\n```\n' + (selected || 'code') + '\n```';
            cursorOffset = selected ? insert.length : 4;
            break;
        case 'hr':
            insert = (before.endsWith('\n') || !before) ? '---\n' : '\n---\n';
            cursorOffset = insert.length;
            break;
        case 'table':
            insert = (before.endsWith('\n') || !before) ? '' : '\n';
            insert += '| Column 1 | Column 2 | Column 3 |\n|----------|----------|----------|\n| Cell 1   | Cell 2   | Cell 3   |\n';
            cursorOffset = (before.endsWith('\n') || !before) ? 2 : 3;
            break;
        case 'mention':
            insert = '@';
            cursorOffset = 1;
            break;
    }

    editor.value = before + insert + after;
    editor.selectionStart = editor.selectionEnd = start + cursorOffset;
    editor.focus();
    updatePreview();
    markDirty();
}

// Keyboard shortcuts for formatting
editor.addEventListener('keydown', (e) => {
    if (e.ctrlKey || e.metaKey) {
        switch(e.key.toLowerCase()) {
            case 'b': e.preventDefault(); insertFormat('bold'); break;
            case 'i': e.preventDefault(); insertFormat('italic'); break;
            case 'k': e.preventDefault(); insertFormat('link'); break;
            case 'h': e.preventDefault(); insertFormat('heading'); break;
        }
    }
});

// View switcher
let currentView = 'split';
function setView(view) {
    currentView = view;
    const layout = document.getElementById('editorLayout');
    layout.classList.remove('split', 'editor-only', 'preview-only');
    if (view !== 'split') layout.classList.add(view);
    document.querySelectorAll('.view-btn').forEach(btn => {
        btn.classList.toggle('active', btn.dataset.view === view);
    });
}
setView('split'); // Initialize

function buildFullContent() {
    const fm = {
        type: frontmatter.type || 'decision',
        id: frontmatter.id || recordId,
        title: fieldTitle.value,
        status: fieldStatus.value,
        created: frontmatter.created || new Date().toISOString().split('T')[0],
        updated: new Date().toISOString().split('T')[0],
        authors: authors,
        tags: tags,
        core: fieldFoundational.checked || undefined,
        links: links
    };

    // Build YAML
    let yaml = '---\n';
    yaml += `type: ${fm.type}\n`;
    yaml += `id: ${fm.id}\n`;
    yaml += `title: ${fm.title.includes(':') ? `"${fm.title}"` : fm.title}\n`;
    yaml += `status: ${fm.status}\n`;
    yaml += `created: ${fm.created}\n`;
    yaml += `updated: ${fm.updated}\n`;
    yaml += `authors: [${fm.authors.join(', ')}]\n`;
    yaml += `tags: [${fm.tags.join(', ')}]\n`;
    if (fm.core) yaml += `core: true\n`;
    yaml += `links:\n`;
    for (const [k, v] of Object.entries(fm.links)) {
        yaml += `  ${k}: [${(v || []).join(', ')}]\n`;
    }
    yaml += '---\n\n';

    return yaml + editor.value;
}

// Server-side markdown rendering to prevent XSS (no client-side regex parsing)
let renderPending = false;
let renderQueued = false;

async function renderMarkdown(md) {
    try {
        const res = await fetch('/api/render', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ markdown: md })
        });
        if (!res.ok) return '<p class="text-red-400">Preview unavailable</p>';
        const data = await res.json();
        return data.html || '';
    } catch {
        return '<p class="text-red-400">Preview unavailable</p>';
    }
}

async function updatePreview() {
    document.getElementById('displayTitle').textContent = fieldTitle.value || 'Untitled';

    // Debounce: if render in progress, queue another
    if (renderPending) {
        renderQueued = true;
        return;
    }

    renderPending = true;
    const html = await renderMarkdown(editor.value);
    preview.innerHTML = html;
    // Render KaTeX math in preview
    renderMath();
    renderPending = false;

    // Process queued render
    if (renderQueued) {
        renderQueued = false;
        updatePreview();
    }
}

// Render math with retry for when KaTeX hasn't loaded yet
function renderMath() {
    if (typeof renderMathInElement === 'function') {
        renderMathInElement(preview, {delimiters: [{left: '$$', right: '$$', display: true}, {left: '$', right: '$', display: false}]});
    } else {
        // KaTeX not loaded yet, retry in 100ms
        setTimeout(renderMath, 100);
    }
}

function markDirty() {
    isDirty = true;
    document.title = '• Edit ' + recordId + ' - {{ site.title }}';
}

function updateCursorPosition() {
    const text = editor.value.substring(0, editor.selectionStart);
    const lines = text.split('\n');
    cursorPos.textContent = `Ln ${lines.length}, Col ${lines[lines.length - 1].length + 1}`;
}

function showStatus(message, type = 'info') {
    const colors = {
        success: 'bg-green-900/50 border-green-700 text-green-300',
        error: 'bg-red-900/50 border-red-700 text-red-300',
        info: 'bg-blue-900/50 border-blue-700 text-blue-300',
        warning: 'bg-yellow-900/50 border-yellow-700 text-yellow-300'
    };
    statusBar.className = `mb-4 px-4 py-2 rounded-lg text-sm border ${colors[type]}`;
    statusBar.textContent = message;
    statusBar.classList.remove('hidden');
    if (type === 'success') setTimeout(() => statusBar.classList.add('hidden'), 3000);
}

async function save() {
    const saveIcon = document.getElementById('saveIcon');
    const saveBtnText = document.getElementById('saveBtnText');
    saveBtn.disabled = true;
    saveIcon.outerHTML = '<svg id="saveIcon" class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/></svg>';
    saveBtnText.textContent = 'Saving...';

    try {
        const content = buildFullContent();
        const res = await fetch(`/api/records/${recordId}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ content })
        });
        const data = await res.json();
        if (res.ok) {
            // Show success state on button
            saveBtn.classList.remove('bg-piper-accent', 'hover:bg-piper-light');
            saveBtn.classList.add('bg-green-600');
            document.getElementById('saveIcon').outerHTML = '<svg id="saveIcon" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>';
            saveBtnText.textContent = 'Saved!';
            originalFull = content;
            isDirty = false;
            document.title = 'Edit ' + recordId + ' - {{ site.title }}';
            // Reset button after 2s
            setTimeout(() => {
                saveBtn.classList.remove('bg-green-600');
                saveBtn.classList.add('bg-piper-accent', 'hover:bg-piper-light');
                saveBtnText.textContent = 'Save';
            }, 2000);
        } else {
            showStatus(data.error || 'Failed to save', 'error');
            resetSaveBtn();
        }
    } catch (err) {
        showStatus('Network error: ' + err.message, 'error');
        resetSaveBtn();
    } finally {
        saveBtn.disabled = false;
    }
}

function resetSaveBtn() {
    const saveIcon = document.getElementById('saveIcon');
    const saveBtnText = document.getElementById('saveBtnText');
    saveBtn.classList.remove('bg-green-600');
    saveBtn.classList.add('bg-piper-accent', 'hover:bg-piper-light');
    saveIcon.outerHTML = '<svg id="saveIcon" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>';
    saveBtnText.textContent = 'Save';
}

// Event listeners
editor.addEventListener('input', () => { updatePreview(); markDirty(); });
editor.addEventListener('keyup', updateCursorPosition);
editor.addEventListener('click', updateCursorPosition);
fieldTitle.addEventListener('input', () => { updatePreview(); markDirty(); });
fieldStatus.addEventListener('change', markDirty);
fieldFoundational.addEventListener('change', markDirty);
saveBtn.addEventListener('click', save);

document.addEventListener('keydown', (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 's') { e.preventDefault(); save(); }
    if (e.key === 'Escape') {
        if (isDirty && !confirm('Unsaved changes. Leave anyway?')) return;
        window.location.href = '/records/' + recordId;
    }
});

editor.addEventListener('keydown', (e) => {
    if (e.key === 'Tab') {
        e.preventDefault();
        const start = editor.selectionStart;
        editor.value = editor.value.substring(0, start) + '  ' + editor.value.substring(editor.selectionEnd);
        editor.selectionStart = editor.selectionEnd = start + 2;
        updatePreview();
    }
});

window.addEventListener('beforeunload', (e) => { if (isDirty) { e.preventDefault(); e.returnValue = ''; } });

// Initialize
initFromRaw();
updateCursorPosition();
</script>
{% endblock %}
"##;
