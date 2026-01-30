use minijinja::Environment;

const BASE_TEMPLATE: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{% block title %}{{ site.title }}{% endblock %}</title>
    {% if site.description %}<meta name="description" content="{{ site.description }}">{% endif %}
    <link rel="stylesheet" href="/static/tailwind.css">
    <link rel="stylesheet" href="/static/katex.min.css">
    <style>
        @font-face {
            font-family: 'Inter';
            font-style: normal;
            font-weight: 100 900;
            font-display: swap;
            src: url('/static/fonts/InterVariable.ttf') format('truetype');
        }
        @font-face {
            font-family: 'JetBrains Mono';
            font-style: normal;
            font-weight: 400;
            font-display: swap;
            src: url('/static/fonts/JetBrainsMono-Regular.woff2') format('woff2');
        }
        :root {
            --bg: #101524;
            --surface: #1a202c;
            --primary: {{ site.primary_color | default(value="#007c43") }};
            --accent: {{ site.accent_color | default(value="#00a55a") }};
            --text: #e2e8f0;
            --text-dim: #94a3b8;
            --success: #4CAF50;
            --warning: #FF9800;
        }
        /* Author avatar and tooltip - CSS only with group-hover */
        .author-wrapper {
            position: relative;
        }
        .author-avatar, .avatar-initials {
            width: 2.5rem;
            height: 2.5rem;
            border-radius: 9999px;
            border: 2px solid #1e293b;
            cursor: pointer;
            transition: border-color 0.2s, transform 0.15s;
        }
        .author-wrapper:hover .author-avatar,
        .author-wrapper:hover .avatar-initials {
            border-color: var(--accent);
            transform: translateY(-2px);
            z-index: 10;
        }
        .author-tooltip {
            position: absolute;
            bottom: 100%;
            left: 50%;
            transform: translateX(-50%);
            margin-bottom: 0.5rem;
            padding: 0.5rem 0.75rem;
            background: #0f172a;
            border: 1px solid #334155;
            border-radius: 0.5rem;
            font-size: 0.75rem;
            color: #e2e8f0;
            white-space: nowrap;
            z-index: 50;
            box-shadow: 0 4px 12px rgba(0,0,0,0.4);
            opacity: 0;
            visibility: hidden;
            transition: opacity 0.15s, visibility 0.15s;
            pointer-events: none;
        }
        .author-wrapper:hover .author-tooltip {
            opacity: 1;
            visibility: visible;
        }
        /* Avatar initials fallback */
        .avatar-initials {
            display: none;
            background: linear-gradient(135deg, var(--primary), var(--accent));
            color: white;
            font-size: 0.875rem;
            font-weight: 600;
            align-items: center;
            justify-content: center;
        }
        .avatar-initials.show {
            display: flex;
        }
        .author-avatar.hidden {
            display: none;
        }
        /* List styles for markdown content only */
        .content ul { list-style-type: disc; padding-left: 1.5rem; margin: 1rem 0; }
        .content ol { list-style-type: decimal; padding-left: 1.5rem; margin: 1rem 0; }
        .content li { margin: 0.25rem 0; }
        {{ site.custom_css | default(value="") | safe }}
    </style>
    <script defer src="/static/katex.min.js"></script>
    <script defer src="/static/auto-render.min.js" onload="renderMathInElement(document.body, {delimiters: [{left: '$$', right: '$$', display: true}, {left: '$', right: '$', display: false}]});"></script>
    {% block head %}{% endblock %}
</head>
<body class="bg-base-300 text-base-content min-h-screen font-sans">
    <header class="navbar bg-base-100 border-b border-neutral px-8">
        <div class="flex-1">
            <a href="/" class="flex items-center gap-3 no-underline text-inherit">
                {% if site.logo %}<img src="{{ site.logo }}" alt="{{ site.title }}" class="h-8">
                {% else %}<span class="text-xl font-bold text-primary-content">{{ site.title }}</span>{% endif %}
            </a>
        </div>
        <nav role="tablist" class="tabs tabs-box">
            <a role="tab" href="/" class="tab{% if current_page == "records" %} tab-active{% endif %}">Records</a>
            <a role="tab" href="/timeline" class="tab{% if current_page == "timeline" %} tab-active{% endif %}">Timeline</a>
            <a role="tab" href="/graph" class="tab{% if current_page == "graph" %} tab-active{% endif %}">Graph</a>
            {% if has_users %}<a role="tab" href="/users" class="tab{% if current_page == "users" %} tab-active{% endif %}">Users</a>
            <a role="tab" href="/teams" class="tab{% if current_page == "teams" %} tab-active{% endif %}">Teams</a>{% endif %}
            <a role="tab" href="/stats" class="tab{% if current_page == "stats" %} tab-active{% endif %}">Stats</a>
        </nav>
    </header>
    <main class="max-w-5xl mx-auto px-8 py-8">
        {% block content %}{% endblock %}
    </main>
    {% if site.footer %}
    <footer class="text-center py-8 text-slate-500 text-sm">{{ site.footer }} ©</footer>
    {% endif %}
    {% block scripts %}{% endblock %}
    <script>
    // Record ID linkification with hover previews
    const quickPreview = {{ site.quick_preview | default(value=true) }};
    const recordCache = {};
    const recordPattern = /\b(DEC|STR|POL|CUS|OPP|PRC|HIR|ADR|INC|RUN|MTG|FBK|LEG)-\d{3}\b/g;

    function linkifyRecordIds() {
        const contentElements = document.querySelectorAll('.content, .card-meta, .preview-meta, td, .link-type');
        contentElements.forEach(el => {
            if (el.querySelector('.record-link')) return; // already processed
            linkifyTextNodes(el);
        });
    }

    function linkifyTextNodes(element) {
        const walker = document.createTreeWalker(element, NodeFilter.SHOW_TEXT, null, false);
        const nodesToReplace = [];
        while (walker.nextNode()) {
            const node = walker.currentNode;
            if (node.parentElement.closest('a, .record-link, code, pre, script, style')) continue;
            if (recordPattern.test(node.textContent)) {
                nodesToReplace.push(node);
            }
            recordPattern.lastIndex = 0;
        }
        nodesToReplace.forEach(node => {
            const fragment = document.createDocumentFragment();
            let lastIndex = 0;
            let match;
            recordPattern.lastIndex = 0;
            while ((match = recordPattern.exec(node.textContent)) !== null) {
                if (match.index > lastIndex) {
                    fragment.appendChild(document.createTextNode(node.textContent.slice(lastIndex, match.index)));
                }
                const link = createRecordLink(match[0]);
                fragment.appendChild(link);
                lastIndex = match.index + match[0].length;
            }
            if (lastIndex < node.textContent.length) {
                fragment.appendChild(document.createTextNode(node.textContent.slice(lastIndex)));
            }
            node.parentNode.replaceChild(fragment, node);
        });
    }

    function createRecordLink(id) {
        const link = document.createElement('a');
        link.href = '/records/' + id;
        link.className = 'record-link';
        link.textContent = id;
        if (quickPreview) {
            link.addEventListener('mouseenter', () => showPreview(link, id));
        }
        return link;
    }

    async function showPreview(link, id) {
        if (link.querySelector('.record-preview')) return;
        let data = recordCache[id];
        if (!data) {
            try {
                const resp = await fetch('/api/records/' + id);
                if (resp.ok) {
                    data = await resp.json();
                    recordCache[id] = data;
                }
            } catch (e) { return; }
        }
        if (!data) return;
        const preview = document.createElement('div');
        preview.className = 'record-preview';
        preview.innerHTML = `
            <div class="preview-title">${data.title}<span class="preview-status ${data.status}">${data.status}</span></div>
            <div class="preview-meta">${data.type_display} | ${data.created}</div>
        `;
        link.appendChild(preview);
        link.addEventListener('mouseleave', () => preview.remove(), { once: true });
    }

    document.addEventListener('DOMContentLoaded', linkifyRecordIds);

    // Author avatar: image error fallback (tooltips are CSS-only)
    document.addEventListener('DOMContentLoaded', function() {
        document.querySelectorAll('.author-avatar').forEach(avatar => {
            avatar.addEventListener('error', function() {
                this.classList.add('hidden');
                const fallback = this.nextElementSibling;
                if (fallback && fallback.classList.contains('avatar-initials')) {
                    fallback.classList.add('show');
                }
            });
        });
    });
    </script>
</body>
</html>
"##;

const INDEX_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}Records - {{ site.title }}{% endblock %}

{% block head %}
<style>
    .filter-btn.active { background: oklch(var(--p) / 0.2); border-color: oklch(var(--p)); color: oklch(var(--s)); }
</style>
{% endblock %}

{% block content %}
<h1 class="text-3xl font-bold mb-6">Records</h1>

<input type="text" class="input input-bordered w-full mb-6" placeholder="Search records..." id="search">

<div class="flex gap-2 mb-6 flex-wrap items-center">
    <button class="btn btn-sm btn-outline filter-btn active" data-type="all">All</button>
    {% for rt in record_types %}
    <button class="btn btn-sm btn-outline filter-btn" data-type="{{ rt.code }}">{{ rt.display }}</button>
    {% endfor %}
    <div id="tagFilter" class="hidden"></div>
    <div class="ml-auto flex gap-2">
        <button id="viewToggle" class="btn btn-sm btn-outline btn-square" title="Toggle view">
            <svg id="viewIconCards" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/></svg>
            <svg id="viewIconTable" class="w-5 h-5 hidden" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16"/></svg>
        </button>
        <button id="sort" class="btn btn-sm btn-outline" title="Newest First">↓</button>
    </div>
</div>

<div id="records" class="grid grid-cols-1 md:grid-cols-2 gap-4">
{% for record in records %}
<a href="/records/{{ record.id }}" class="card card-border bg-base-100 hover:bg-base-200 transition-all hover:-translate-y-0.5 {% if record.core %}border-l-4 border-l-warning{% endif %}" data-type="{{ record.type }}" data-status="{{ record.status }}" data-id="{{ record.id }}" data-created="{{ record.created }}" data-core="{{ record.core }}" data-tags="{{ record.tags | join(',') }}">
    <div class="card-body p-5">
        <div class="flex justify-between items-start gap-3">
            <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-1">
                    <span class="font-mono text-xs opacity-40">{{ record.id }}</span>
                    {% if record.is_draft %}<span class="badge badge-xs badge-secondary badge-outline">DRAFT</span>{% endif %}
                    {% if record.core %}<span class="badge badge-xs badge-warning badge-outline">CORE</span>{% endif %}
                </div>
                <h3 class="text-base font-semibold text-base-content mb-2">{{ record.title }}</h3>
                <div class="flex flex-wrap items-center gap-x-2 gap-y-1 text-xs opacity-50">
                    <span class="badge badge-xs badge-ghost">{{ record.type_display }}</span>
                    {% if record.status == 'deprecated' %}<span class="text-warning">{{ record.created_year }} → {{ record.updated_year }}</span>{% elif record.type == 'INC' and record.status == 'open' %}<span class="text-error">{{ record.created_year }} → ongoing</span>{% elif record.type == 'INC' and record.status == 'resolved' %}<span class="text-info">{{ record.created_year }} → {{ record.updated_year }}</span>{% else %}<span>{{ record.created }}</span>{% endif %}
                </div>
                {% if record.tags %}
                <div class="flex flex-wrap gap-1 mt-2">{% for tag in record.tags %}<span class="tag-link badge badge-xs badge-outline opacity-60 cursor-pointer hover:opacity-100" data-tag="{{ tag }}">#{{ tag }}</span>{% endfor %}</div>
                {% endif %}
            </div>
            <span class="badge badge-sm flex-shrink-0 {% if record.status == 'accepted' or record.status == 'active' %}badge-success{% elif record.status == 'proposed' or record.status == 'draft' %}badge-warning{% elif record.status == 'open' %}badge-error{% elif record.status == 'resolved' %}badge-info{% elif record.status == 'deprecated' %}badge-warning badge-outline{% elif record.status == 'superseded' %}badge-neutral{% else %}badge-neutral{% endif %}">{{ record.status | upper }}</span>
        </div>
    </div>
</a>
{% endfor %}
</div>

<div id="recordsTable" class="hidden overflow-x-auto">
    <table class="table table-zebra">
        <thead>
            <tr>
                <th>ID</th>
                <th>Title</th>
                <th>Type</th>
                <th>Status</th>
                <th>Date</th>
            </tr>
        </thead>
        <tbody>
            {% for record in records %}
            <tr class="table-row hover:bg-base-200 cursor-pointer {% if record.core %}bg-warning/5{% endif %}" data-type="{{ record.type }}" data-status="{{ record.status }}" data-id="{{ record.id }}" data-created="{{ record.created }}" data-core="{{ record.core }}" data-tags="{{ record.tags | join(',') }}" data-href="/records/{{ record.id }}">
                <td class="font-mono text-xs whitespace-nowrap">
                    <span class="opacity-50">{{ record.id }}</span>
                    {% if record.is_draft %}<span class="badge badge-xs badge-secondary ml-1">D</span>{% endif %}
                    {% if record.core %}<span class="badge badge-xs badge-warning ml-1">★</span>{% endif %}
                </td>
                <td class="font-medium">{{ record.title }}</td>
                <td class="text-sm opacity-60">{{ record.type_display }}</td>
                <td>
                    <span class="badge badge-sm {% if record.status == 'accepted' or record.status == 'active' %}badge-success{% elif record.status == 'proposed' or record.status == 'draft' %}badge-warning{% elif record.status == 'open' %}badge-error{% elif record.status == 'resolved' %}badge-info{% elif record.status == 'deprecated' %}badge-warning badge-outline{% elif record.status == 'superseded' %}badge-neutral{% else %}badge-neutral{% endif %}">{{ record.status }}</span>
                </td>
                <td class="text-sm opacity-60 whitespace-nowrap">{{ record.created }}</td>
            </tr>
            {% endfor %}
        </tbody>
    </table>
</div>
{% endblock %}

{% block scripts %}
<script>
const search = document.getElementById('search');
const recordsContainer = document.getElementById('records');
const recordsTable = document.getElementById('recordsTable');
const filters = document.querySelectorAll('.filter-btn');
const sortBtn = document.getElementById('sort');
const tagFilterEl = document.getElementById('tagFilter');
const viewToggle = document.getElementById('viewToggle');
const viewIconCards = document.getElementById('viewIconCards');
const viewIconTable = document.getElementById('viewIconTable');
let activeType = 'all';
let activeStatus = 'all';
let activeTag = '';
let sortMode = 'newest'; // newest -> oldest -> core -> newest
let viewMode = localStorage.getItem('dg-view-mode') || 'cards';

// View toggle
function setViewMode(mode) {
    viewMode = mode;
    localStorage.setItem('dg-view-mode', mode);
    if (mode === 'table') {
        recordsContainer.classList.add('hidden');
        recordsTable.classList.remove('hidden');
        viewIconCards.classList.add('hidden');
        viewIconTable.classList.remove('hidden');
    } else {
        recordsContainer.classList.remove('hidden');
        recordsTable.classList.add('hidden');
        viewIconCards.classList.remove('hidden');
        viewIconTable.classList.add('hidden');
    }
}

viewToggle.addEventListener('click', () => {
    setViewMode(viewMode === 'cards' ? 'table' : 'cards');
});

// Make table rows clickable
document.querySelectorAll('.table-row').forEach(row => {
    row.addEventListener('click', () => {
        window.location.href = row.dataset.href;
    });
});

const sortModes = {
    newest: { next: 'oldest', icon: '↓', title: 'Newest First' },
    oldest: { next: 'core', icon: '↑', title: 'Oldest First' },
    core: { next: 'newest', icon: '★', title: 'Core First' }
};

// Tag filter UI
function updateTagFilterUI() {
    if (activeTag) {
        tagFilterEl.innerHTML = `<button class="btn btn-sm btn-primary" onclick="clearTag()">
            <span>#${activeTag}</span>
            <span class="opacity-60 hover:text-error text-lg leading-none">&times;</span>
        </button>`;
        tagFilterEl.classList.remove('hidden');
    } else {
        tagFilterEl.classList.add('hidden');
    }
}

function clearTag() {
    activeTag = '';
    updateTagFilterUI();
    filterRecords();
    updateUrl();
}

// URL state management
function updateUrl() {
    const params = new URLSearchParams();
    if (search.value) params.set('q', search.value);
    if (activeType !== 'all') params.set('type', activeType);
    if (activeStatus !== 'all') params.set('status', activeStatus);
    if (activeTag) params.set('tag', activeTag);
    if (sortMode !== 'newest') params.set('sort', sortMode);
    const url = params.toString() ? '?' + params.toString() : '/';
    history.replaceState(null, '', url);
}

function loadFromUrl() {
    const params = new URLSearchParams(window.location.search);
    if (params.get('q')) search.value = params.get('q');
    if (params.get('type')) {
        activeType = params.get('type');
        filters.forEach(b => {
            if (b.id !== 'sort' && b.tagName === 'BUTTON') {
                b.classList.toggle('active', b.dataset.type === activeType);
            }
        });
    }
    if (params.get('status')) {
        activeStatus = params.get('status');
    }
    if (params.get('tag')) {
        activeTag = params.get('tag');
        updateTagFilterUI();
    }
    if (params.get('sort') && sortModes[params.get('sort')]) {
        sortMode = params.get('sort');
        sortBtn.innerHTML = sortModes[sortMode].icon;
        sortBtn.title = sortModes[sortMode].title;
    }
}

search.addEventListener('input', () => { filterRecords(); updateUrl(); });
sortBtn.addEventListener('click', cycleSortMode);
filters.forEach(btn => {
    if (btn.id !== 'sort' && btn.id !== 'viewToggle' && btn.tagName === 'BUTTON' && btn.dataset.type) {
        btn.addEventListener('click', () => {
            filters.forEach(b => { if (b.id !== 'sort' && b.id !== 'viewToggle' && b.tagName === 'BUTTON' && b.dataset.type) b.classList.remove('active'); });
            btn.classList.add('active');
            activeType = btn.dataset.type;
            filterRecords();
            updateUrl();
        });
    }
});

function cycleSortMode() {
    sortMode = sortModes[sortMode].next;
    sortBtn.innerHTML = sortModes[sortMode].icon;
    sortBtn.title = sortModes[sortMode].title;
    sortRecords();
    updateUrl();
}

function filterRecords() {
    const query = search.value.toLowerCase();
    // Filter cards
    document.querySelectorAll('.card').forEach(r => {
        const matchesType = activeType === 'all' || r.dataset.type === activeType;
        const matchesStatus = activeStatus === 'all' || r.dataset.status === activeStatus;
        const matchesQuery = !query || r.textContent.toLowerCase().includes(query);
        const tags = r.dataset.tags ? r.dataset.tags.split(',') : [];
        const matchesTag = !activeTag || tags.includes(activeTag);
        r.style.display = matchesType && matchesStatus && matchesQuery && matchesTag ? 'block' : 'none';
    });
    // Filter table rows
    document.querySelectorAll('.table-row').forEach(r => {
        const matchesType = activeType === 'all' || r.dataset.type === activeType;
        const matchesStatus = activeStatus === 'all' || r.dataset.status === activeStatus;
        const matchesQuery = !query || r.textContent.toLowerCase().includes(query);
        const tags = r.dataset.tags ? r.dataset.tags.split(',') : [];
        const matchesTag = !activeTag || tags.includes(activeTag);
        r.style.display = matchesType && matchesStatus && matchesQuery && matchesTag ? 'table-row' : 'none';
    });
}

function sortRecords() {
    const sortFn = (a, b) => {
        if (sortMode === 'newest') {
            return b.dataset.created.localeCompare(a.dataset.created);
        } else if (sortMode === 'oldest') {
            return a.dataset.created.localeCompare(b.dataset.created);
        } else { // core
            const aF = a.dataset.core === 'true';
            const bF = b.dataset.core === 'true';
            if (aF !== bF) return bF - aF;
            return b.dataset.created.localeCompare(a.dataset.created);
        }
    };
    // Sort cards
    const cards = Array.from(recordsContainer.querySelectorAll('.card'));
    cards.sort(sortFn);
    cards.forEach(card => recordsContainer.appendChild(card));
    // Sort table rows
    const tbody = recordsTable.querySelector('tbody');
    const rows = Array.from(tbody.querySelectorAll('.table-row'));
    rows.sort(sortFn);
    rows.forEach(row => tbody.appendChild(row));
}

// Handle tag clicks
document.querySelectorAll('.tag-link').forEach(tag => {
    tag.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        activeTag = tag.dataset.tag;
        updateTagFilterUI();
        filterRecords();
        updateUrl();
    });
});

// Initialize from URL on page load
loadFromUrl();
setViewMode(viewMode);
filterRecords();
sortRecords();

// Keyboard navigation
let selectedIndex = -1;

function getVisibleRecords() {
    if (viewMode === 'cards') {
        return Array.from(recordsContainer.querySelectorAll('.card')).filter(r => r.style.display !== 'none');
    } else {
        return Array.from(recordsTable.querySelectorAll('.table-row')).filter(r => r.style.display !== 'none');
    }
}

function updateSelection(newIndex) {
    const records = getVisibleRecords();
    if (records.length === 0) return;

    // Remove previous selection
    records.forEach(r => r.classList.remove('ring-2', 'ring-piper-accent', 'ring-offset-2', 'ring-offset-piper-bg'));

    // Clamp index
    selectedIndex = Math.max(0, Math.min(newIndex, records.length - 1));

    // Add selection to new item
    const selected = records[selectedIndex];
    if (selected) {
        selected.classList.add('ring-2', 'ring-piper-accent', 'ring-offset-2', 'ring-offset-piper-bg');
        selected.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
    }
}

function openSelected() {
    const records = getVisibleRecords();
    if (selectedIndex >= 0 && selectedIndex < records.length) {
        const selected = records[selectedIndex];
        const href = selected.href || selected.dataset.href;
        if (href) window.location.href = href;
    }
}

document.addEventListener('keydown', (e) => {
    // Don't interfere with input fields
    if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') {
        if (e.key === 'Escape') {
            e.target.blur();
            e.preventDefault();
        }
        return;
    }

    switch(e.key) {
        case 'j':
        case 'ArrowDown':
            e.preventDefault();
            updateSelection(selectedIndex + 1);
            break;
        case 'k':
        case 'ArrowUp':
            e.preventDefault();
            updateSelection(selectedIndex - 1);
            break;
        case 'Enter':
            if (selectedIndex >= 0) {
                e.preventDefault();
                openSelected();
            }
            break;
        case '/':
            e.preventDefault();
            search.focus();
            break;
        case 'g':
            // gg = go to top
            if (e.timeStamp - (window.lastG || 0) < 500) {
                e.preventDefault();
                updateSelection(0);
            }
            window.lastG = e.timeStamp;
            break;
        case 'G':
            // G = go to bottom
            e.preventDefault();
            updateSelection(getVisibleRecords().length - 1);
            break;
        case 'Escape':
            selectedIndex = -1;
            getVisibleRecords().forEach(r => r.classList.remove('ring-2', 'ring-piper-accent', 'ring-offset-2', 'ring-offset-piper-bg'));
            break;
    }
});
</script>
{% endblock %}
"##;

const RECORD_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}{{ record.id }} - {{ site.title }}{% endblock %}

{% block content %}
<div class="w-full bg-piper-card border border-slate-700 rounded-2xl shadow-2xl overflow-hidden">
    <!-- Accent bar -->
    <div class="h-1.5 w-full bg-gradient-to-r from-piper-accent to-emerald-400"></div>

    <div class="p-8 pb-4">
        <!-- Header row -->
        <div class="flex justify-between items-start mb-4">
            <div class="flex items-center gap-3">
                <span class="font-mono text-sm font-medium text-slate-400 bg-slate-800 px-2 py-1 rounded border border-slate-700">
                    {{ record.id }}
                </span>
                {% if record.core %}
                <span class="flex items-center gap-1.5 px-3 py-1 rounded-full bg-yellow-900/30 border border-yellow-800/30 text-yellow-500 text-xs font-semibold uppercase tracking-wide">
                    <span class="w-2 h-2 rounded-full bg-yellow-500"></span>
                    CORE
                </span>
                {% endif %}
                <span class="flex items-center gap-1.5 px-3 py-1 rounded-full bg-piper-accent/20 text-piper-light text-xs font-semibold uppercase tracking-wide">
                    <span class="w-2 h-2 rounded-full bg-piper-light {% if record.status == 'accepted' or record.status == 'active' %}animate-pulse{% endif %}"></span>
                    {{ record.status }}
                </span>
            </div>
            <div class="flex gap-2">
                <a href="/records/{{ record.id }}/edit" class="p-2 hover:bg-slate-700 rounded-lg transition-colors text-slate-400 hover:text-white" title="Edit">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path></svg>
                </a>
                <a href="/graph?focus={{ record.id }}" class="p-2 hover:bg-slate-700 rounded-lg transition-colors text-slate-400 hover:text-white" title="View Graph">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path></svg>
                </a>
            </div>
        </div>

        <!-- Title -->
        <h1 class="text-3xl font-bold text-white mb-4 leading-tight">{{ record.title }}</h1>

        <!-- Meta row -->
        <div class="flex flex-wrap items-center gap-6 text-sm text-slate-400 border-b border-slate-700/50 pb-6">
            <div class="flex items-center gap-2">
                <svg class="w-4 h-4 text-slate-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                {% if record.status == 'deprecated' %}<span class="text-amber-400/70 font-medium">{{ record.created_year }} → {{ record.updated_year }}</span>{% elif record.type == 'INC' and record.status == 'open' %}<span class="text-red-400 font-medium">{{ record.created_year }} → ongoing</span>{% elif record.type == 'INC' and record.status == 'resolved' %}<span class="text-blue-400/70 font-medium">{{ record.created_year }} → {{ record.updated_year }}</span>{% else %}<span>{{ record.created }}</span>{% endif %}
            </div>
            <div class="flex items-center gap-2">
                <span class="text-xs font-mono uppercase tracking-wider text-slate-500">{{ record.type_display }}</span>
            </div>
            {% if record.resolved_authors %}
            <div class="flex items-center gap-3">
                <span class="text-xs font-mono uppercase tracking-wider text-slate-500">Authors:</span>
                <div class="flex -space-x-3">
                    {% for author in record.resolved_authors %}
                    <div class="author-wrapper">
                        <a href="/users/{{ author.username }}">
                            <img src="{{ author.avatar_url }}" alt="{{ author.name }}" class="author-avatar bg-piper-accent" data-initials="{{ author.initials }}">
                            <span class="avatar-initials" data-initials="{{ author.initials }}">{{ author.initials }}</span>
                        </a>
                        <div class="author-tooltip">
                            <div class="font-medium">{{ author.name }}</div>
                            {% if author.email %}<div class="text-slate-400 text-[11px]">{{ author.email }}</div>{% endif %}
                            {% if author.teams %}<div class="text-slate-500 text-[10px] mt-1">{% for t in author.teams %}<a href="/teams/{{ t }}" class="hover:text-piper-light">{{ t }}</a>{% if not loop.last %}, {% endif %}{% endfor %}</div>{% endif %}
                        </div>
                    </div>
                    {% endfor %}
                </div>
            </div>
            {% elif record.authors %}
            <div class="flex items-center gap-3">
                <span class="text-xs font-mono uppercase tracking-wider text-slate-500">Authors:</span>
                <span class="text-slate-300">{{ record.authors | join(", ") }}</span>
            </div>
            {% endif %}
            {% if record.tags %}
            <div class="flex flex-wrap gap-2">
                {% for tag in record.tags %}
                <a href="/?tag={{ tag }}" class="px-2 py-1 bg-slate-800 rounded text-xs text-slate-300 font-mono hover:bg-piper-accent hover:text-white transition-colors no-underline">#{{ tag }}</a>
                {% endfor %}
            </div>
            {% endif %}
        </div>

        <!-- Content with ToC -->
        <div class="mt-6 flex gap-8">
            <div class="flex-1 text-slate-300 leading-relaxed max-w-3xl content" id="content">
                {{ record.content_html | safe }}
            </div>
            <nav id="toc" class="hidden lg:block w-56 shrink-0">
                <div class="sticky top-6">
                    <button id="toc-toggle" class="flex items-center gap-1.5 text-xs font-bold text-slate-500 uppercase tracking-widest mb-4 font-mono hover:text-slate-400 transition-colors">
                        <svg id="toc-chevron" class="w-2.5 h-2.5 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M19 9l-7 7-7-7"></path></svg>
                        On This Page
                    </button>
                    <ul id="toc-list" class="list-none space-y-1 text-sm"></ul>
                </div>
            </nav>
        </div>
    </div>

    {% if record.links or record.backlinks %}
    <!-- Connections section -->
    <div class="bg-slate-800/30 border-t border-slate-700 p-8">
        {% if record.links %}
        <h3 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-6 font-mono">Links To</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-8">
            {% for link in record.links %}
            <a href="/records/{{ link.target }}" class="group block p-4 bg-slate-800 border border-slate-700 rounded-xl hover:border-piper-light/50 hover:bg-slate-700/50 transition-all hover:shadow-lg hover:-translate-y-0.5">
                <div class="flex justify-between items-start mb-1">
                    <span class="font-mono text-xs text-piper-light font-medium">{{ link.target }}</span>
                    <span class="text-[10px] uppercase font-bold text-slate-500 border border-slate-600 px-1 rounded">{{ link.type }}</span>
                </div>
                {% if link.title %}
                <div class="font-semibold text-slate-200 group-hover:text-white">{{ link.title }}</div>
                {% endif %}
            </a>
            {% endfor %}
        </div>
        {% endif %}

        {% if record.backlinks %}
        <h3 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-6 font-mono">Referenced By</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {% for link in record.backlinks %}
            <a href="/records/{{ link.source }}" class="group block p-4 bg-slate-800 border border-slate-700 rounded-xl hover:border-amber-500/50 hover:bg-slate-700/50 transition-all hover:shadow-lg hover:-translate-y-0.5">
                <div class="flex justify-between items-start mb-1">
                    <span class="font-mono text-xs text-amber-400 font-medium">{{ link.source }}</span>
                    <span class="text-[10px] uppercase font-bold text-slate-500 border border-slate-600 px-1 rounded">{{ link.type }}</span>
                </div>
                {% if link.title %}
                <div class="font-semibold text-slate-200 group-hover:text-white">{{ link.title }}</div>
                {% endif %}
            </a>
            {% endfor %}
        </div>
        {% endif %}
    </div>
    {% endif %}

    <!-- Footer -->
    <div class="bg-slate-900 p-4 border-t border-slate-800 flex justify-between items-center text-xs text-slate-500 font-mono">
        <span>{{ record.id }}</span>
        {% if record.resolved_authors %}<span>Authors: {% for a in record.resolved_authors %}{{ a.name }}{% if not loop.last %}, {% endif %}{% endfor %}</span>{% elif record.authors %}<span>Authors: {{ record.authors | join(", ") }}</span>{% endif %}
    </div>
</div>
{% endblock %}

{% block scripts %}
<script>
(function() {
    const content = document.getElementById('content');
    const tocList = document.getElementById('toc-list');
    const toc = document.getElementById('toc');

    // Find all h2 and h3 headings
    const headings = content.querySelectorAll('h2, h3');
    if (headings.length < 2) {
        toc.style.display = 'none';
        return;
    }

    // Generate ToC items
    const tocItems = [];
    headings.forEach((heading, index) => {
        // Add ID to heading if it doesn't have one
        if (!heading.id) {
            heading.id = 'heading-' + index;
        }

        const li = document.createElement('li');
        const a = document.createElement('a');
        a.href = '#' + heading.id;
        a.textContent = heading.textContent;
        a.className = 'block py-1.5 pl-4 text-slate-400 hover:text-slate-200 hover:border-slate-500 transition-colors border-l-2 border-slate-800';
        if (heading.tagName === 'H3') {
            li.classList.add('ml-4');
            a.classList.add('text-xs');
        }
        a.addEventListener('click', (e) => {
            e.preventDefault();
            heading.scrollIntoView({ behavior: 'smooth', block: 'start' });
            history.pushState(null, '', '#' + heading.id);
        });
        li.appendChild(a);
        tocList.appendChild(li);
        tocItems.push({ heading, link: a });
    });

    // Highlight current section on scroll
    function updateActiveSection() {
        const scrollPos = window.scrollY + 100;
        let activeIndex = 0;

        for (let i = 0; i < tocItems.length; i++) {
            if (tocItems[i].heading.offsetTop <= scrollPos) {
                activeIndex = i;
            }
        }

        tocItems.forEach((item, index) => {
            if (index === activeIndex) {
                item.link.classList.remove('text-slate-400', 'border-slate-800');
                item.link.classList.add('text-slate-100', 'border-piper-accent', 'font-medium');
            } else {
                item.link.classList.add('text-slate-400', 'border-slate-800');
                item.link.classList.remove('text-slate-100', 'border-piper-accent', 'font-medium');
            }
        });
    }

    window.addEventListener('scroll', updateActiveSection, { passive: true });
    updateActiveSection();

    // TOC collapse toggle
    const tocToggle = document.getElementById('toc-toggle');
    const tocChevron = document.getElementById('toc-chevron');
    let tocCollapsed = false;
    tocToggle.addEventListener('click', () => {
        tocCollapsed = !tocCollapsed;
        tocList.style.display = tocCollapsed ? 'none' : 'block';
        tocChevron.style.transform = tocCollapsed ? 'rotate(-90deg)' : 'rotate(0)';
    });

    // Mark Pros/Cons lists with appropriate classes
    content.querySelectorAll('p').forEach(p => {
        const strong = p.querySelector('strong:only-child');
        if (!strong) return;
        const text = strong.textContent.trim().toLowerCase();
        const nextEl = p.nextElementSibling;
        if (nextEl && nextEl.tagName === 'UL') {
            if (text.includes('pros')) {
                nextEl.classList.add('pros-list');
            } else if (text.includes('cons')) {
                nextEl.classList.add('cons-list');
            }
        }
    });
})();
</script>
{% endblock %}
"##;

const GRAPH_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}Graph - {{ site.title }}{% endblock %}

{% block head %}
<script src="https://d3js.org/d3.v7.min.js"></script>
<style>
#graph { width: 100%; height: 600px; }
#graph svg { width: 100%; height: 100%; }
.graph-tooltip {
    position: fixed;
    background: var(--surface, #1a202c);
    border: 1px solid var(--primary, #007c43);
    border-radius: 8px;
    padding: 12px 16px;
    font-size: 13px;
    pointer-events: none;
    z-index: 1000;
    box-shadow: 0 4px 16px rgba(0,0,0,0.4);
    display: none;
    max-width: 300px;
    color: var(--text, #e2e8f0);
}
.graph-tooltip .tooltip-type {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-dim, #94a3b8);
    margin-bottom: 4px;
}
.graph-tooltip .tooltip-title {
    font-weight: 600;
    font-size: 14px;
    margin-bottom: 8px;
    line-height: 1.3;
}
.graph-tooltip .tooltip-meta {
    font-size: 12px;
    color: var(--text-dim, #94a3b8);
    display: flex;
    flex-direction: column;
    gap: 4px;
}
</style>
{% endblock %}

{% block content %}
<div class="graph-container" id="graph"></div>
{% endblock %}

{% block scripts %}
<script>
const data = {{ graph_data | safe }};
const container = document.getElementById('graph');
const width = container.clientWidth;
const height = container.clientHeight || 600;

const color = d3.scaleOrdinal()
    .domain(['DEC', 'STR', 'POL', 'CUS', 'OPP', 'PRC', 'HIR', 'ADR', 'INC', 'RUN', 'MTG'])
    .range(['#4CAF50', '#2196F3', '#FF9800', '#9C27B0', '#E91E63', '#00BCD4', '#795548', '#607D8B', '#F44336', '#8BC34A', '#03A9F4']);

const svg = d3.select('#graph')
    .append('svg')
    .attr('viewBox', [0, 0, width, height]);

// Add zoom behavior
const g = svg.append('g');
const zoom = d3.zoom()
    .scaleExtent([0.1, 4])
    .on('zoom', (e) => g.attr('transform', e.transform));
svg.call(zoom);

const simulation = d3.forceSimulation(data.nodes)
    .force('link', d3.forceLink(data.edges).id(d => d.id).distance(100))
    .force('charge', d3.forceManyBody().strength(-300))
    .force('center', d3.forceCenter(width / 2, height / 2));

const link = g.append('g')
    .selectAll('line')
    .data(data.edges)
    .join('line')
    .attr('stroke', '#666')
    .attr('stroke-width', 1);

const node = g.append('g')
    .selectAll('g')
    .data(data.nodes)
    .join('g')
    .call(d3.drag()
        .on('start', dragstarted)
        .on('drag', dragged)
        .on('end', dragended));

node.append('circle')
    .attr('r', 28)
    .attr('fill', d => color(d.type))
    .attr('stroke', d => d.core ? 'gold' : '#333')
    .attr('stroke-width', d => d.core ? 3 : 1);

node.append('text')
    .text(d => d.id)
    .attr('text-anchor', 'middle')
    .attr('dy', 5)
    .attr('fill', '#fff')
    .attr('font-size', '11px')
    .attr('font-weight', '500');

// Custom tooltip
const tooltip = d3.select('body').append('div').attr('class', 'graph-tooltip');
node.on('mouseenter', (e, d) => {
    const authors = d.authors && d.authors.length > 0 ? d.authors.join(', ') : 'Unknown';
    const date = d.date || 'No date';
    tooltip.html(`
        <div class="tooltip-type">${d.type_name || d.type}</div>
        <div class="tooltip-title">${d.title}</div>
        <div class="tooltip-meta">
            <span>📅 ${date}</span>
            <span>👤 ${authors}</span>
        </div>
    `).style('display', 'block');
}).on('mousemove', (e) => {
    tooltip.style('left', (e.clientX + 12) + 'px').style('top', (e.clientY + 12) + 'px');
}).on('mouseleave', () => {
    tooltip.style('display', 'none');
});
node.on('click', (e, d) => window.location.href = '/records/' + d.id);
node.style('cursor', 'pointer');

simulation.on('tick', () => {
    link.attr('x1', d => d.source.x).attr('y1', d => d.source.y)
        .attr('x2', d => d.target.x).attr('y2', d => d.target.y);
    node.attr('transform', d => `translate(${d.x},${d.y})`);
});

// Fit all nodes in view after simulation settles
simulation.on('end', fitToView);

function fitToView() {
    if (data.nodes.length === 0) return;
    const padding = 40;
    const nodeRadius = 28;
    let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
    data.nodes.forEach(d => {
        minX = Math.min(minX, d.x - nodeRadius);
        maxX = Math.max(maxX, d.x + nodeRadius);
        minY = Math.min(minY, d.y - nodeRadius);
        maxY = Math.max(maxY, d.y + nodeRadius);
    });
    const bw = maxX - minX + padding * 2;
    const bh = maxY - minY + padding * 2;
    const scale = Math.min(width / bw, height / bh, 1);
    const tx = (width - bw * scale) / 2 - (minX - padding) * scale;
    const ty = (height - bh * scale) / 2 - (minY - padding) * scale;
    svg.transition().duration(500).call(zoom.transform, d3.zoomIdentity.translate(tx, ty).scale(scale));
}

function dragstarted(e) { if (!e.active) simulation.alphaTarget(0.3).restart(); e.subject.fx = e.subject.x; e.subject.fy = e.subject.y; }
function dragged(e) { e.subject.fx = e.x; e.subject.fy = e.y; }
function dragended(e) { if (!e.active) simulation.alphaTarget(0); e.subject.fx = null; e.subject.fy = null; }
</script>
{% endblock %}
"##;

const TIMELINE_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}Timeline - {{ site.title }}{% endblock %}

{% block head %}
<style>
    .timeline-container {
        position: relative;
        padding: 2rem 0;
        overflow-x: auto;
    }
    .timeline-svg {
        display: block;
        margin: 0 auto;
    }
    .timeline-node {
        cursor: pointer;
    }
    .timeline-node circle {
        transition: r 0.15s ease-out, stroke-width 0.15s ease-out;
    }
    .timeline-node:hover circle {
        stroke-width: 3;
    }
    .timeline-node text {
        pointer-events: none;
    }
    .year-label {
        fill: var(--text-dim);
        font-size: 14px;
        font-weight: bold;
    }
    .year-line {
        stroke: var(--primary);
        stroke-width: 1;
        stroke-dasharray: 4,4;
    }
    .dependency-line {
        fill: none;
        stroke-width: 2;
        opacity: 0.6;
        transition: opacity 0.2s;
    }
    .timeline-svg.hover-active .dependency-line { opacity: 0.1; }
    .timeline-svg.hover-active .dependency-line.highlight { opacity: 1; stroke-width: 3; }
    .timeline-svg.hover-active .timeline-node { opacity: 0.3; transition: opacity 0.2s; }
    .timeline-svg.hover-active .timeline-node.highlight { opacity: 1; }
    .timeline-svg.hover-active line[class="trunk-line"],
    .timeline-svg.hover-active line:not([class]) { opacity: 0.1; transition: opacity 0.2s; }
    .timeline-tooltip {
        position: fixed;
        background: var(--surface);
        border: 1px solid var(--primary);
        border-radius: 6px;
        padding: 0.5rem 0.75rem;
        font-size: 0.85rem;
        pointer-events: none;
        z-index: 1000;
        box-shadow: 0 4px 12px rgba(0,0,0,0.4);
        display: none;
        color: var(--text);
    }
    .trunk-line {
        stroke: var(--text-dim);
        stroke-width: 3;
    }
    .legend {
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;
        margin-bottom: 1.5rem;
        justify-content: center;
    }
    .legend-item {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 0.85rem;
    }
    .legend-dot {
        width: 12px;
        height: 12px;
        border-radius: 50%;
    }
    .gap-label {
        fill: var(--text-dim);
        font-size: 12px;
        font-style: italic;
    }
    .gap-dots {
        fill: var(--text-dim);
        font-size: 16px;
        letter-spacing: 2px;
    }
</style>
{% endblock %}

{% block content %}
<h2 style="margin-bottom: 1rem; text-align: center;">Decision Timeline</h2>

<div class="legend" id="legend"></div>

<div class="timeline-container">
    <svg id="timeline" class="timeline-svg"></svg>
    <div id="timeline-tooltip" class="timeline-tooltip"></div>
</div>
{% endblock %}

{% block scripts %}
<script>
const data = {{ timeline_data | safe }};
const records = data.nodes;
const edges = data.edges;

// Type colors matching graph view
const typeColors = {
    DEC: '#4CAF50', STR: '#2196F3', POL: '#FF9800', CUS: '#9C27B0',
    OPP: '#E91E63', PRC: '#00BCD4', HIR: '#795548', ADR: '#607D8B',
    INC: '#F44336', RUN: '#8BC34A', MTG: '#03A9F4'
};

const typeNames = {
    DEC: 'Decision', STR: 'Strategy', POL: 'Policy', CUS: 'Customer',
    OPP: 'Opportunity', PRC: 'Process', HIR: 'Hiring', ADR: 'Architecture',
    INC: 'Incident', RUN: 'Runbook', MTG: 'Meeting'
};

// Build legend
const legend = document.getElementById('legend');
const usedTypes = [...new Set(records.map(r => r.type))].sort();
usedTypes.forEach(type => {
    const item = document.createElement('div');
    item.className = 'legend-item';
    item.innerHTML = `<span class="legend-dot" style="background: ${typeColors[type] || '#666'}"></span><span>${typeNames[type] || type}</span>`;
    legend.appendChild(item);
});

// Parse dates and sort
records.forEach(r => {
    r.date = new Date(r.created);
    r.year = r.date.getFullYear();
});
records.sort((a, b) => a.date - b.date);

// Get year range
const minYear = Math.min(...records.map(r => r.year));
const maxYear = Math.max(...records.map(r => r.year));
const years = [];
for (let y = minYear; y <= maxYear; y++) years.push(y);

// Layout config
const margin = { top: 60, right: 60, bottom: 40, left: 80 };
const nodeRadius = 22;  // Larger radius to fit IDs like DEC-001
const nodeSpacing = 60; // Minimum vertical spacing between nodes

// Assign lanes (columns) to avoid overlap - git-style
const lanes = {};  // type -> lane index
const typeOrder = ['DEC', 'STR', 'POL', 'ADR', 'INC', 'RUN', 'PRC', 'HIR', 'CUS', 'OPP', 'MTG'];
typeOrder.forEach((t, i) => lanes[t] = i);

// Get unique types in our data and assign lanes
const dataTypes = [...new Set(records.map(r => r.type))];
dataTypes.sort((a, b) => (typeOrder.indexOf(a) !== -1 ? typeOrder.indexOf(a) : 99) - (typeOrder.indexOf(b) !== -1 ? typeOrder.indexOf(b) : 99));
dataTypes.forEach((t, i) => lanes[t] = i);

const laneWidth = 80;  // Wider lanes for larger nodes
const width = margin.left + (dataTypes.length) * laneWidth + margin.right;

// Group records by year and calculate year heights based on record count
const recordsByYear = {};
years.forEach(y => recordsByYear[y] = []);
records.forEach(r => recordsByYear[r.year].push(r));

// Identify year gaps (consecutive empty years) and create display items
const displayItems = [];  // { type: 'year', year } or { type: 'gap', startYear, endYear }
let gapStart = null;
years.forEach((y, i) => {
    const hasRecords = recordsByYear[y].length > 0;
    if (!hasRecords) {
        if (gapStart === null) gapStart = y;
    } else {
        if (gapStart !== null) {
            const gapEnd = years[i - 1];
            displayItems.push({ type: 'gap', startYear: gapStart, endYear: gapEnd });
            gapStart = null;
        }
        displayItems.push({ type: 'year', year: y });
    }
});
// Handle trailing gap
if (gapStart !== null) {
    displayItems.push({ type: 'gap', startYear: gapStart, endYear: years[years.length - 1] });
}

// Calculate heights for display items
const itemHeights = {};
const minYearHeight = 120;
const gapHeight = 50;  // Compact height for year gaps
displayItems.forEach(item => {
    if (item.type === 'gap') {
        itemHeights[`gap-${item.startYear}`] = gapHeight;
    } else {
        const count = recordsByYear[item.year].length;
        itemHeights[item.year] = Math.max(minYearHeight, count * nodeSpacing + 40);
    }
});

// For backward compatibility, also populate yearHeights for years with records
const yearHeights = {};
years.forEach(y => {
    if (recordsByYear[y].length > 0) {
        yearHeights[y] = itemHeights[y];
    }
});

// Calculate cumulative Y offsets
const yearOffsets = {};
const gapOffsets = {};
let cumulativeY = margin.top;
displayItems.forEach(item => {
    if (item.type === 'gap') {
        gapOffsets[item.startYear] = { y: cumulativeY, startYear: item.startYear, endYear: item.endYear };
        cumulativeY += gapHeight;
    } else {
        yearOffsets[item.year] = cumulativeY;
        cumulativeY += itemHeights[item.year];
    }
});

const height = cumulativeY + margin.bottom;

// Calculate positions - spread nodes evenly within each year section
// Only process years that have records
const yearsWithRecords = years.filter(y => recordsByYear[y].length > 0);
yearsWithRecords.forEach(year => {
    const yearRecords = recordsByYear[year];

    // Sort by date within year, then by type for consistent ordering
    yearRecords.sort((a, b) => a.date - b.date || lanes[a.type] - lanes[b.type]);

    const sectionHeight = yearHeights[year];
    const startY = yearOffsets[year] + 30; // Padding from year line
    const availableHeight = sectionHeight - 60; // Leave padding at bottom

    yearRecords.forEach((r, i) => {
        // Spread nodes evenly within the year section
        const progress = yearRecords.length === 1 ? 0.5 : i / (yearRecords.length - 1);
        r.y = startY + progress * availableHeight;
        r.x = margin.left + lanes[r.type] * laneWidth + laneWidth / 2;
    });
});

// Build ID to record map
const recordMap = {};
records.forEach(r => recordMap[r.id] = r);

// Create SVG
const svg = document.getElementById('timeline');
svg.setAttribute('width', width);
svg.setAttribute('height', height);
svg.setAttribute('viewBox', `0 0 ${width} ${height}`);

// Draw year separators, labels, and gap indicators
displayItems.forEach(item => {
    if (item.type === 'gap') {
        // Draw gap indicator
        const gapInfo = gapOffsets[item.startYear];
        const y = gapInfo.y;
        const centerY = y + gapHeight / 2;

        // Dashed line across
        const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
        line.setAttribute('x1', margin.left - 20);
        line.setAttribute('y1', y);
        line.setAttribute('x2', width - margin.right + 20);
        line.setAttribute('y2', y);
        line.setAttribute('class', 'year-line');
        line.setAttribute('opacity', 0.3);
        svg.appendChild(line);

        // Gap label with year range
        const label = document.createElementNS('http://www.w3.org/2000/svg', 'text');
        label.setAttribute('x', 20);
        label.setAttribute('y', centerY);
        label.setAttribute('class', 'gap-label');
        const rangeText = item.startYear === item.endYear
            ? item.startYear
            : `${item.startYear}–${item.endYear}`;
        label.textContent = rangeText;
        svg.appendChild(label);

        // Vertical dots in the center
        const dots = document.createElementNS('http://www.w3.org/2000/svg', 'text');
        dots.setAttribute('x', width / 2);
        dots.setAttribute('y', centerY + 4);
        dots.setAttribute('text-anchor', 'middle');
        dots.setAttribute('class', 'gap-dots');
        dots.textContent = '···';
        svg.appendChild(dots);
    } else {
        // Draw regular year
        const year = item.year;
        const y = yearOffsets[year];

        // Year line
        const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
        line.setAttribute('x1', margin.left - 20);
        line.setAttribute('y1', y);
        line.setAttribute('x2', width - margin.right + 20);
        line.setAttribute('y2', y);
        line.setAttribute('class', 'year-line');
        svg.appendChild(line);

        // Year label
        const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
        text.setAttribute('x', 20);
        text.setAttribute('y', y + yearHeights[year] / 2);
        text.setAttribute('class', 'year-label');
        text.textContent = year;
        svg.appendChild(text);
    }
});

// Draw trunk lines for each lane (vertical lines like git branches)
dataTypes.forEach((type, i) => {
    const x = margin.left + lanes[type] * laneWidth + laneWidth / 2;
    const typeRecords = records.filter(r => r.type === type);
    if (typeRecords.length < 2) return;

    const minY = Math.min(...typeRecords.map(r => r.y));
    const maxY = Math.max(...typeRecords.map(r => r.y));

    const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
    line.setAttribute('x1', x);
    line.setAttribute('y1', minY);
    line.setAttribute('x2', x);
    line.setAttribute('y2', maxY);
    line.setAttribute('stroke', typeColors[type] || '#666');
    line.setAttribute('stroke-width', 2);
    line.setAttribute('opacity', 0.3);
    line.setAttribute('class', 'trunk-line');
    svg.appendChild(line);
});

// Draw dependency edges (curved lines like git merge)
const edgePaths = [];
edges.forEach(edge => {
    const source = recordMap[edge.source];
    const target = recordMap[edge.target];
    if (!source || !target) return;

    const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    const midY = (source.y + target.y) / 2;
    const d = `M ${source.x} ${source.y} C ${source.x} ${midY}, ${target.x} ${midY}, ${target.x} ${target.y}`;
    path.setAttribute('d', d);
    path.setAttribute('class', 'dependency-line');
    path.setAttribute('stroke', typeColors[source.type] || '#666');
    svg.appendChild(path);
    edgePaths.push({ source: edge.source, target: edge.target, pathEl: path });
});

// Build edge index for hover highlighting (populated after edges are drawn)
const edgeIndex = {};
records.forEach(r => edgeIndex[r.id] = []);
edgePaths.forEach(ep => {
    if (edgeIndex[ep.source]) edgeIndex[ep.source].push(ep);
    if (edgeIndex[ep.target]) edgeIndex[ep.target].push(ep);
});

// Draw nodes
const nodeElements = {};
const pathElements = [];
records.forEach(r => {
    const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
    g.setAttribute('class', 'timeline-node');
    g.setAttribute('data-id', r.id);
    g.setAttribute('transform', `translate(${r.x}, ${r.y})`);
    g.onclick = () => window.location.href = '/records/' + r.id;

    const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
    circle.setAttribute('r', nodeRadius);
    circle.setAttribute('fill', typeColors[r.type] || '#666');
    circle.setAttribute('stroke', r.core ? 'gold' : '#333');
    circle.setAttribute('stroke-width', r.core ? 3 : 1);
    g.appendChild(circle);

    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    text.setAttribute('text-anchor', 'middle');
    text.setAttribute('dy', 4);
    text.setAttribute('fill', '#fff');
    text.setAttribute('font-size', '10px');
    text.setAttribute('font-family', 'monospace');
    text.setAttribute('font-weight', 'bold');
    text.textContent = r.id;
    g.appendChild(text);

    nodeElements[r.id] = g;
    svg.appendChild(g);

    // Hover highlighting
    const tooltip = document.getElementById('timeline-tooltip');
    const svgRect = () => svg.getBoundingClientRect();
    g.addEventListener('mouseenter', () => {
        svg.classList.add('hover-active');
        g.classList.add('highlight');
        // Highlight connected nodes and edges
        const connected = edgeIndex[r.id] || [];
        connected.forEach(c => {
            c.pathEl.classList.add('highlight');
            const otherId = c.source === r.id ? c.target : c.source;
            if (nodeElements[otherId]) nodeElements[otherId].classList.add('highlight');
        });
        tooltip.innerHTML = `<strong>${r.id}</strong>: ${r.title}<br><span style="color:var(--text-dim)">${r.type} | ${r.created}</span>`;
        tooltip.style.display = 'block';
        // Position tooltip relative to node
        const rect = svgRect();
        tooltip.style.left = (rect.left + r.x + nodeRadius + 8) + 'px';
        tooltip.style.top = (rect.top + r.y - 10 + window.scrollY) + 'px';
    });
    g.addEventListener('mouseleave', () => {
        svg.classList.remove('hover-active');
        svg.querySelectorAll('.highlight').forEach(el => el.classList.remove('highlight'));
        tooltip.style.display = 'none';
    });
});
</script>
{% endblock %}
"##;

const STATS_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}Stats - {{ site.title }}{% endblock %}

{% block content %}
<h1 class="text-3xl font-bold mb-6">Statistics</h1>

<div class="stats stats-vertical sm:stats-horizontal shadow w-full mb-8">
    <div class="stat">
        <div class="stat-title">Total Records</div>
        <div class="stat-value text-primary">{{ stats.total_records }}</div>
    </div>
    <div class="stat">
        <div class="stat-title">Total Links</div>
        <div class="stat-value text-secondary">{{ stats.total_edges }}</div>
    </div>
    <div class="stat">
        <div class="stat-title">Core Records</div>
        <div class="stat-value text-warning">{{ stats.core }}</div>
    </div>
</div>

<h3 class="text-lg font-semibold mb-4">By Type</h3>
<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3 mb-8">
    {% for item in stats.by_type %}
    <a href="/?type={{ item.type }}" class="card card-border bg-base-100 hover:bg-base-200 hover:-translate-y-0.5 transition-all">
        <div class="card-body p-4 items-center text-center">
            <div class="text-2xl font-bold text-primary">{{ item.count }}</div>
            <div class="text-sm opacity-60">{{ item.type_display }}</div>
        </div>
    </a>
    {% endfor %}
</div>

<h3 class="text-lg font-semibold mb-4">By Status</h3>
<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
    {% for item in stats.by_status %}
    <a href="/?status={{ item.status }}" class="card card-border bg-base-100 hover:bg-base-200 hover:-translate-y-0.5 transition-all">
        <div class="card-body p-4 items-center text-center">
            <div class="text-2xl font-bold text-primary">{{ item.count }}</div>
            <div class="text-sm opacity-60 capitalize">{{ item.status }}</div>
        </div>
    </a>
    {% endfor %}
</div>
{% endblock %}
"##;

const EDIT_TEMPLATE: &str = r##"{% extends "base.html" %}

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

const USERS_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}Users - {{ site.title }}{% endblock %}

{% block content %}
<div class="flex justify-between items-center mb-6">
    <h2 class="text-2xl font-bold text-white">Users</h2>
    <label class="flex items-center gap-2 text-sm text-slate-400 cursor-pointer">
        <input type="checkbox" id="showDeprecated" class="rounded bg-slate-800 border-slate-600 text-piper-accent focus:ring-piper-accent">
        Show deprecated
    </label>
</div>

<div id="users" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
{% for user in users %}
<a href="/users/{{ user.username }}" class="user-card block bg-piper-card border border-slate-700 rounded-xl p-4 hover:border-piper-light/50 hover:bg-slate-700/30 transition-all hover:-translate-y-0.5{% if user.is_deprecated %} opacity-50{% endif %}" data-deprecated="{{ user.is_deprecated }}">
    <div class="flex items-center gap-4">
        <img src="{{ user.avatar_url }}" alt="{{ user.name }}" class="w-12 h-12 rounded-full border-2 border-slate-700" onerror="this.src='https://ui-avatars.com/api/?name={{ user.initials }}&background=007c43&color=fff&size=64'">
        <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
                <span class="font-semibold text-white truncate">{{ user.name }}</span>
                {% if user.is_llm %}
                <span class="px-1.5 py-0.5 rounded text-xs font-semibold bg-purple-900/50 border border-purple-700 text-purple-300">🤖</span>
                {% endif %}
                {% if user.is_deprecated %}
                <span class="px-2 py-0.5 rounded text-xs font-semibold bg-slate-700 text-slate-400">LEFT</span>
                {% endif %}
            </div>
            <div class="text-sm text-slate-400">@{{ user.username }}</div>
            {% if user.teams %}
            <div class="flex flex-wrap gap-1 mt-1">
                {% for team in user.teams %}
                <span class="px-1.5 py-0.5 bg-slate-800 rounded text-xs text-slate-400">{{ team }}</span>
                {% endfor %}
            </div>
            {% endif %}
        </div>
    </div>
</a>
{% endfor %}
</div>

{% if users | length == 0 %}
<div class="text-center text-slate-500 py-12">
    No users configured. Add users to <code class="bg-slate-800 px-2 py-1 rounded">dg.toml</code>
</div>
{% endif %}
{% endblock %}

{% block scripts %}
<script>
const showDeprecated = document.getElementById('showDeprecated');
showDeprecated.addEventListener('change', () => {
    document.querySelectorAll('.user-card').forEach(card => {
        if (card.dataset.deprecated === 'true') {
            card.style.display = showDeprecated.checked ? 'block' : 'none';
        }
    });
});
// Hide deprecated by default
document.querySelectorAll('.user-card[data-deprecated="true"]').forEach(c => c.style.display = 'none');
</script>
{% endblock %}
"##;

const USER_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}{{ user.name }} - {{ site.title }}{% endblock %}

{% block content %}
<div class="max-w-3xl mx-auto">
    <div class="bg-piper-card border border-slate-700 rounded-2xl overflow-hidden">
        <div class="h-1.5 w-full bg-gradient-to-r from-piper-accent to-emerald-400"></div>

        <div class="p-8">
            <div class="flex items-start gap-6">
                <img src="{{ user.avatar_url }}" alt="{{ user.name }}" class="w-24 h-24 rounded-full border-4 border-slate-700" onerror="this.src='https://ui-avatars.com/api/?name={{ user.initials }}&background=007c43&color=fff&size=96'">
                <div class="flex-1">
                    <div class="flex items-center gap-3 mb-2">
                        <h1 class="text-3xl font-bold text-white">{{ user.name }}</h1>
                        {% if "llm" in user.roles %}
                        <span class="px-3 py-1 rounded-full text-sm font-semibold bg-purple-900/50 border border-purple-700 text-purple-300">
                            <span class="mr-1">🤖</span>AI
                        </span>
                        {% endif %}
                        {% if user.is_deprecated %}
                        <span class="px-3 py-1 rounded-full text-sm font-semibold bg-slate-700 text-slate-400">LEFT</span>
                        {% endif %}
                    </div>
                    <div class="text-lg text-slate-400 mb-4">@{{ user.username }}</div>

                    {% if user.email %}
                    <div class="flex items-center gap-2 text-sm text-slate-400 mb-2">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>
                        {{ user.email }}
                    </div>
                    {% endif %}

                    {% if user.github %}
                    <div class="flex items-center gap-2 text-sm text-slate-400 mb-2">
                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
                        <a href="https://github.com/{{ user.github }}" class="text-piper-light hover:underline" target="_blank">{{ user.github }}</a>
                    </div>
                    {% endif %}
                </div>
            </div>

            {% if user.is_deprecated %}
            <div class="mt-6 p-4 bg-slate-800 rounded-lg border border-slate-700">
                <div class="text-sm text-slate-400">
                    <strong class="text-slate-300">Departed:</strong> {{ user.deprecated_date | default(value="Unknown") }}
                    {% if user.deprecated_note %}
                    <br><strong class="text-slate-300">Note:</strong> {{ user.deprecated_note }}
                    {% endif %}
                </div>
            </div>
            {% endif %}

            {% if user.teams %}
            <div class="mt-6">
                <h3 class="text-xs font-mono uppercase tracking-wider text-slate-500 mb-3">Teams</h3>
                <div class="flex flex-wrap gap-2">
                    {% for team in user.teams %}
                    <a href="/teams/{{ team }}" class="px-3 py-1.5 bg-slate-800 rounded-lg text-sm text-slate-300 hover:bg-piper-accent hover:text-white transition-colors">{{ team }}</a>
                    {% endfor %}
                </div>
            </div>
            {% endif %}

            {% if user.roles %}
            <div class="mt-6">
                <h3 class="text-xs font-mono uppercase tracking-wider text-slate-500 mb-3">Roles</h3>
                <div class="flex flex-wrap gap-2">
                    {% for role in user.roles %}
                    <span class="px-3 py-1.5 bg-piper-accent/20 border border-piper-accent/30 rounded-lg text-sm text-piper-light">{{ role }}</span>
                    {% endfor %}
                </div>
            </div>
            {% endif %}
        </div>
    </div>

    {% if authored_records %}
    <div class="mt-8">
        <h3 class="text-lg font-semibold text-white mb-4">Authored Records ({{ authored_records | length }})</h3>
        <div class="space-y-3">
            {% for record in authored_records %}
            <a href="/records/{{ record.id }}" class="block bg-piper-card border border-slate-700 rounded-xl p-4 hover:border-piper-light/50 transition-all">
                <div class="flex justify-between items-start">
                    <div>
                        <span class="font-mono text-sm text-piper-light">{{ record.id }}</span>
                        <span class="ml-2 text-slate-300">{{ record.title }}</span>
                    </div>
                    <span class="px-2 py-0.5 rounded text-xs font-semibold uppercase {% if record.status == 'accepted' %}bg-green-900/30 text-green-500{% else %}bg-slate-700 text-slate-400{% endif %}">{{ record.status }}</span>
                </div>
            </a>
            {% endfor %}
        </div>
    </div>
    {% endif %}

    {% if mentioned_in %}
    <div class="mt-8">
        <h3 class="text-lg font-semibold text-white mb-4">Mentioned In ({{ mentioned_in | length }})</h3>
        <div class="space-y-3">
            {% for record in mentioned_in %}
            <a href="/records/{{ record.id }}" class="block bg-piper-card border border-slate-700 rounded-xl p-4 hover:border-amber-500/50 transition-all">
                <div class="flex justify-between items-start">
                    <div>
                        <span class="font-mono text-sm text-amber-400">{{ record.id }}</span>
                        <span class="ml-2 text-slate-300">{{ record.title }}</span>
                    </div>
                    <span class="px-2 py-0.5 rounded text-xs font-semibold uppercase bg-slate-700 text-slate-400">{{ record.status }}</span>
                </div>
            </a>
            {% endfor %}
        </div>
    </div>
    {% endif %}
</div>
{% endblock %}
"##;

const TEAMS_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}Teams - {{ site.title }}{% endblock %}

{% block content %}
<h2 class="text-2xl font-bold text-white mb-6">Teams</h2>

<div id="teams" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
{% for team in teams %}
<a href="/teams/{{ team.id }}" class="block bg-piper-card border border-slate-700 rounded-xl p-4 hover:border-piper-light/50 hover:bg-slate-700/30 transition-all hover:-translate-y-0.5">
    <div class="flex items-center gap-4">
        <div class="w-12 h-12 rounded-lg bg-gradient-to-br from-piper-accent to-emerald-400 flex items-center justify-center text-white font-bold text-lg">
            {{ team.name | first | upper }}
        </div>
        <div class="flex-1 min-w-0">
            <div class="font-semibold text-white truncate">{{ team.name }}</div>
            {% if team.lead %}
            <div class="text-sm text-slate-400">Lead: @{{ team.lead }}</div>
            {% endif %}
            {% if team.parent %}
            <div class="text-xs text-slate-500 mt-1">↳ {{ team.parent }}</div>
            {% endif %}
            {% if team.member_count > 0 %}
            <div class="text-xs text-slate-500 mt-1">{{ team.member_count }} members</div>
            {% endif %}
        </div>
    </div>
</a>
{% endfor %}
</div>

{% if teams | length == 0 %}
<div class="text-center text-slate-500 py-12">
    No teams configured. Add teams to <code class="bg-slate-800 px-2 py-1 rounded">dg.toml</code>
</div>
{% endif %}
{% endblock %}
"##;

const TEAM_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}{{ team.name }} - {{ site.title }}{% endblock %}

{% block content %}
<div class="max-w-3xl mx-auto">
    <div class="bg-piper-card border border-slate-700 rounded-2xl overflow-hidden">
        <div class="h-1.5 w-full bg-gradient-to-r from-piper-accent to-emerald-400"></div>

        <div class="p-8">
            <div class="flex items-start gap-6">
                <div class="w-24 h-24 rounded-xl bg-gradient-to-br from-piper-accent to-emerald-400 flex items-center justify-center text-white font-bold text-3xl">
                    {{ team.name | first | upper }}
                </div>
                <div class="flex-1">
                    <h1 class="text-3xl font-bold text-white mb-2">{{ team.name }}</h1>
                    <div class="text-lg text-slate-400 mb-4">{{ team.id }}</div>

                    {% if team.description %}
                    <p class="text-slate-300 mb-4">{{ team.description }}</p>
                    {% endif %}

                    {% if team.lead %}
                    <div class="flex items-center gap-2 text-sm text-slate-400 mb-2">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/></svg>
                        Lead: <a href="/users/{{ team.lead }}" class="text-piper-light hover:underline">@{{ team.lead }}</a>
                    </div>
                    {% endif %}

                    {% if team.email %}
                    <div class="flex items-center gap-2 text-sm text-slate-400 mb-2">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>
                        {{ team.email }}
                    </div>
                    {% endif %}

                    {% if team.parent %}
                    <div class="flex items-center gap-2 text-sm text-slate-400">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/></svg>
                        Parent: <a href="/teams/{{ team.parent }}" class="text-piper-light hover:underline">{{ team.parent }}</a>
                    </div>
                    {% endif %}

                    <div class="mt-4">
                        <a href="/teams/{{ team.id }}/history" class="inline-flex items-center gap-2 text-sm text-piper-light hover:text-piper-accent transition-colors">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                            View History
                        </a>
                    </div>
                </div>
            </div>
        </div>
    </div>

    {% if members %}
    <div class="mt-8">
        <h3 class="text-lg font-semibold text-white mb-4">Members ({{ members | length }})</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            {% for member in members %}
            <a href="/users/{{ member.username }}" class="flex items-center gap-3 bg-piper-card border border-slate-700 rounded-xl p-3 hover:border-piper-light/50 transition-all">
                <img src="{{ member.avatar_url }}" alt="{{ member.name }}" class="w-10 h-10 rounded-full border-2 border-slate-700">
                <div>
                    <div class="text-white font-medium">{{ member.name }}</div>
                    <div class="text-sm text-slate-400">@{{ member.username }}</div>
                </div>
            </a>
            {% endfor %}
        </div>
    </div>
    {% endif %}

    {% if sub_teams %}
    <div class="mt-8">
        <h3 class="text-lg font-semibold text-white mb-4">Sub-teams ({{ sub_teams | length }})</h3>
        <div class="space-y-3">
            {% for sub in sub_teams %}
            <a href="/teams/{{ sub.id }}" class="block bg-piper-card border border-slate-700 rounded-xl p-4 hover:border-piper-light/50 transition-all">
                <div class="font-semibold text-white">{{ sub.name }}</div>
                {% if sub.lead %}
                <div class="text-sm text-slate-400">Lead: @{{ sub.lead }}</div>
                {% endif %}
            </a>
            {% endfor %}
        </div>
    </div>
    {% endif %}
</div>
{% endblock %}
"##;

const TEAM_HISTORY_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}{{ team_name }} History - {{ site.title }}{% endblock %}

{% block content %}
<div class="max-w-4xl mx-auto">
    <div class="mb-6">
        <a href="/teams/{{ team_id }}" class="text-piper-light hover:underline text-sm">← Back to {{ team_name }}</a>
    </div>

    <div class="bg-piper-card border border-slate-700 rounded-2xl overflow-hidden">
        <div class="h-1.5 w-full bg-gradient-to-r from-piper-accent to-emerald-400"></div>

        <div class="p-8">
            <h1 class="text-2xl font-bold text-white mb-2">{{ team_name }} History</h1>
            <p class="text-slate-400 mb-6">Team membership changes over time from git history</p>

            {% if history %}
            <div class="space-y-6">
                {% for snapshot in history %}
                <div class="relative pl-6 border-l-2 border-slate-700 {% if loop.first %}border-l-emerald-500{% endif %}">
                    <div class="absolute -left-2 top-0 w-4 h-4 rounded-full {% if loop.first %}bg-emerald-500{% else %}bg-slate-600{% endif %}"></div>

                    <div class="mb-2">
                        <span class="text-slate-300 font-medium">{{ snapshot.date }}</span>
                        <span class="text-slate-600 mx-2">·</span>
                        <span class="font-mono text-xs text-slate-500">{{ snapshot.commit }}</span>
                    </div>

                    <p class="text-sm text-slate-400 mb-3">{{ snapshot.message }}</p>

                    {% if snapshot.joined %}
                    <div class="flex flex-wrap gap-2 mb-2">
                        {% for user in snapshot.joined %}
                        <span class="inline-flex items-center rounded-md bg-green-500/10 px-2 py-1 text-xs font-medium text-green-400 ring-1 ring-inset ring-green-500/20">
                            + @{{ user }}
                        </span>
                        {% endfor %}
                    </div>
                    {% endif %}

                    {% if snapshot.left %}
                    <div class="flex flex-wrap gap-2 mb-2">
                        {% for user in snapshot.left %}
                        <span class="inline-flex items-center rounded-md bg-red-500/10 px-2 py-1 text-xs font-medium text-red-400 ring-1 ring-inset ring-red-500/20">
                            − @{{ user }}
                        </span>
                        {% endfor %}
                    </div>
                    {% endif %}

                    <div class="text-xs text-slate-500">
                        Members: {% for user in snapshot.members %}<a href="/users/{{ user }}" class="text-slate-400 hover:text-piper-light">@{{ user }}</a>{% if not loop.last %}, {% endif %}{% endfor %}
                    </div>
                </div>
                {% endfor %}
            </div>
            {% else %}
            <div class="text-center py-12 text-slate-500">
                <p>No history found for this team.</p>
                <p class="text-sm mt-2">Team membership changes are tracked when dg.toml is committed to git.</p>
            </div>
            {% endif %}
        </div>
    </div>

    {% if all_time_members %}
    <div class="mt-8 bg-piper-card border border-slate-700 rounded-xl p-6">
        <h3 class="text-lg font-semibold text-white mb-4">All-Time Members ({{ all_time_members | length }})</h3>
        <div class="flex flex-wrap gap-2">
            {% for user in all_time_members %}
            <a href="/users/{{ user }}" class="inline-flex items-center rounded-md bg-slate-700/50 px-3 py-1.5 text-sm text-slate-300 hover:bg-slate-600 transition-colors">
                @{{ user }}
            </a>
            {% endfor %}
        </div>
    </div>
    {% endif %}
</div>
{% endblock %}
"##;

pub fn create_environment() -> Environment<'static> {
    let mut env = Environment::new();
    env.add_template("base.html", BASE_TEMPLATE).unwrap();
    env.add_template("index.html", INDEX_TEMPLATE).unwrap();
    env.add_template("record.html", RECORD_TEMPLATE).unwrap();
    env.add_template("graph.html", GRAPH_TEMPLATE).unwrap();
    env.add_template("stats.html", STATS_TEMPLATE).unwrap();
    env.add_template("timeline.html", TIMELINE_TEMPLATE)
        .unwrap();
    env.add_template("edit.html", EDIT_TEMPLATE).unwrap();
    env.add_template("users.html", USERS_TEMPLATE).unwrap();
    env.add_template("user.html", USER_TEMPLATE).unwrap();
    env.add_template("teams.html", TEAMS_TEMPLATE).unwrap();
    env.add_template("team.html", TEAM_TEMPLATE).unwrap();
    env.add_template("team_history.html", TEAM_HISTORY_TEMPLATE)
        .unwrap();
    env
}
