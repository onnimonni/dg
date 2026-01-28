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
        {{ site.custom_css | default(value="") | safe }}
    </style>
    <script defer src="/static/katex.min.js"></script>
    <script defer src="/static/auto-render.min.js" onload="renderMathInElement(document.body, {delimiters: [{left: '$$', right: '$$', display: true}, {left: '$', right: '$', display: false}]});"></script>
    {% block head %}{% endblock %}
</head>
<body class="bg-piper-dark text-slate-300 min-h-screen font-sans">
    <header class="bg-piper-card border-b border-slate-700 px-8 py-4 flex justify-between items-center">
        <a href="/" class="flex items-center gap-3 no-underline text-inherit">
            {% if site.logo %}<img src="{{ site.logo }}" alt="{{ site.title }}" class="h-8">
            {% else %}<h1 class="text-xl font-bold text-white">{{ site.title }}</h1>{% endif %}
        </a>
        <nav class="flex gap-1">
            <a href="/" class="px-3 py-1.5 rounded-lg text-slate-300 hover:text-white hover:bg-slate-700 transition-colors{% if current_page == "records" %} bg-piper-accent text-white{% endif %}">Records</a>
            <a href="/timeline" class="px-3 py-1.5 rounded-lg text-slate-300 hover:text-white hover:bg-slate-700 transition-colors{% if current_page == "timeline" %} bg-piper-accent text-white{% endif %}">Timeline</a>
            <a href="/graph" class="px-3 py-1.5 rounded-lg text-slate-300 hover:text-white hover:bg-slate-700 transition-colors{% if current_page == "graph" %} bg-piper-accent text-white{% endif %}">Graph</a>
            <a href="/stats" class="px-3 py-1.5 rounded-lg text-slate-300 hover:text-white hover:bg-slate-700 transition-colors{% if current_page == "stats" %} bg-piper-accent text-white{% endif %}">Stats</a>
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
    const recordPattern = /\b(DEC|STR|POL|CUS|OPP|PRC|HIR|ADR|INC|RUN|MTG)-\d{3}\b/g;

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
    .filter-btn.active { background: #007c43; border-color: #007c43; color: white; }
</style>
{% endblock %}

{% block content %}
<input type="text" class="w-full px-4 py-3 border border-slate-700 rounded-xl bg-piper-card text-slate-300 mb-6 focus:outline-none focus:border-piper-accent" placeholder="Search records..." id="search">

<div class="flex gap-2 mb-6 flex-wrap items-center">
    <button class="filter-btn px-4 py-2 border border-slate-700 rounded-lg bg-transparent text-slate-300 cursor-pointer hover:bg-slate-700 transition-colors active" data-type="all">All</button>
    {% for rt in record_types %}
    <button class="filter-btn px-4 py-2 border border-slate-700 rounded-lg bg-transparent text-slate-300 cursor-pointer hover:bg-slate-700 transition-colors" data-type="{{ rt.code }}">{{ rt.display }}</button>
    {% endfor %}
    <div id="tagFilter" class="hidden"></div>
    <button id="sort" class="filter-btn px-4 py-2 border border-slate-700 rounded-lg bg-transparent text-slate-300 cursor-pointer hover:bg-slate-700 transition-colors ml-auto" title="Core First">★</button>
</div>

<div id="records" class="space-y-3">
{% for record in records %}
<a href="/records/{{ record.id }}" class="card block bg-piper-card border border-slate-700 rounded-xl p-4 hover:border-piper-light/50 hover:bg-slate-700/30 transition-all hover:-translate-y-0.5 {% if record.foundational %}border-l-4 border-l-yellow-500{% endif %}" data-type="{{ record.type }}" data-status="{{ record.status }}" data-id="{{ record.id }}" data-created="{{ record.created }}" data-foundational="{{ record.foundational }}" data-tags="{{ record.tags | join(',') }}">
    <div class="flex justify-between items-start mb-2">
        <div class="flex items-center gap-2">
            <span class="font-mono text-sm font-medium text-piper-light">{{ record.id }}</span>
            {% if record.foundational %}<span class="px-2 py-0.5 rounded text-xs font-semibold bg-yellow-900/30 text-yellow-500 border border-yellow-800/30">CORE</span>{% endif %}
        </div>
        <span class="px-2 py-0.5 rounded text-xs font-semibold uppercase {% if record.status == 'accepted' or record.status == 'active' %}bg-green-900/30 text-green-500{% elif record.status == 'proposed' or record.status == 'draft' %}bg-yellow-900/30 text-yellow-500{% elif record.status == 'open' %}bg-red-900/30 text-red-500{% elif record.status == 'resolved' %}bg-blue-900/30 text-blue-500{% else %}bg-slate-700 text-slate-400{% endif %}">{{ record.status }}</span>
    </div>
    <div class="text-lg font-semibold text-slate-200 hover:text-white mb-1">{{ record.title }}</div>
    <div class="text-sm text-slate-500 flex items-center gap-2">
        <span>{{ record.type_display }}</span>
        <span class="text-slate-600">·</span>
        <span>{{ record.created }}</span>
        {% if record.tags %}
        <span class="text-slate-600">·</span>
        {% for tag in record.tags %}<span class="tag-link px-1.5 py-0.5 bg-slate-800 rounded text-xs text-slate-400 font-mono hover:bg-piper-accent hover:text-white transition-colors" data-tag="{{ tag }}">#{{ tag }}</span>{% endfor %}
        {% endif %}
    </div>
</a>
{% endfor %}
</div>
{% endblock %}

{% block scripts %}
<script>
const search = document.getElementById('search');
const recordsContainer = document.getElementById('records');
const filters = document.querySelectorAll('.filter-btn');
const sortBtn = document.getElementById('sort');
const tagFilterEl = document.getElementById('tagFilter');
let activeType = 'all';
let activeStatus = 'all';
let activeTag = '';
let sortMode = 'default'; // default -> newest -> oldest -> default

const sortModes = {
    default: { next: 'newest', icon: '★', title: 'Core First' },
    newest: { next: 'oldest', icon: '↓', title: 'Newest First' },
    oldest: { next: 'default', icon: '↑', title: 'Oldest First' }
};

// Tag filter UI
function updateTagFilterUI() {
    if (activeTag) {
        tagFilterEl.innerHTML = `<button class="filter-btn px-4 py-2 border rounded-lg transition-colors flex items-center gap-2 active border-piper-accent bg-piper-accent/20 text-piper-light" onclick="clearTag()">
            <span>#${activeTag}</span>
            <span class="text-piper-light/60 hover:text-red-400 text-lg leading-none">&times;</span>
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
    if (sortMode !== 'default') params.set('sort', sortMode);
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
    if (btn.id !== 'sort' && btn.tagName === 'BUTTON') {
        btn.addEventListener('click', () => {
            filters.forEach(b => { if (b.id !== 'sort' && b.tagName === 'BUTTON') b.classList.remove('active'); });
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
    document.querySelectorAll('.card').forEach(r => {
        const matchesType = activeType === 'all' || r.dataset.type === activeType;
        const matchesStatus = activeStatus === 'all' || r.dataset.status === activeStatus;
        const matchesQuery = !query || r.textContent.toLowerCase().includes(query);
        const tags = r.dataset.tags ? r.dataset.tags.split(',') : [];
        const matchesTag = !activeTag || tags.includes(activeTag);
        r.style.display = matchesType && matchesStatus && matchesQuery && matchesTag ? 'block' : 'none';
    });
}

function sortRecords() {
    const cards = Array.from(recordsContainer.querySelectorAll('.card'));
    cards.sort((a, b) => {
        if (sortMode === 'default') {
            const aF = a.dataset.foundational === 'true';
            const bF = b.dataset.foundational === 'true';
            if (aF !== bF) return bF - aF;
            return b.dataset.created.localeCompare(a.dataset.created);
        } else if (sortMode === 'newest') {
            return b.dataset.created.localeCompare(a.dataset.created);
        } else {
            return a.dataset.created.localeCompare(b.dataset.created);
        }
    });
    cards.forEach(card => recordsContainer.appendChild(card));
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
filterRecords();
sortRecords();
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
                {% if record.foundational %}
                <span class="flex items-center gap-1.5 px-3 py-1 rounded-full bg-yellow-900/30 border border-yellow-800/30 text-yellow-500 text-xs font-semibold uppercase tracking-wide">
                    <span class="w-2 h-2 rounded-full bg-yellow-500"></span>
                    CORE
                </span>
                {% endif %}
                <span class="flex items-center gap-1.5 px-3 py-1 rounded-full bg-piper-accent/10 border border-piper-accent/30 text-piper-light text-xs font-semibold uppercase tracking-wide">
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
                <span>{{ record.created }}</span>
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
                        <img src="{{ author.avatar_url }}" alt="{{ author.name }}" class="author-avatar bg-piper-accent" data-initials="{{ author.initials }}">
                        <span class="avatar-initials" data-initials="{{ author.initials }}">{{ author.initials }}</span>
                        <div class="author-tooltip">
                            <div class="font-medium">{{ author.name }}</div>
                            {% if author.email %}<div class="text-slate-400 text-[11px]">{{ author.email }}</div>{% endif %}
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
            <div class="flex gap-2 ml-auto">
                {% for tag in record.tags %}
                <a href="/?tag={{ tag }}" class="px-2 py-1 bg-slate-800 rounded text-xs text-slate-300 font-mono hover:bg-piper-accent hover:text-white transition-colors no-underline">#{{ tag }}</a>
                {% endfor %}
            </div>
            {% endif %}
        </div>

        <!-- Content preview -->
        <div class="mt-6 text-slate-300 leading-relaxed max-w-3xl content">
            {{ record.content_html | safe }}
        </div>
    </div>

    {% if record.links %}
    <!-- Connections section -->
    <div class="bg-slate-800/30 border-t border-slate-700 p-8">
        <h3 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-6 font-mono">Decision Graph Connections</h3>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
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
    </div>
    {% endif %}

    <!-- Footer -->
    <div class="bg-slate-900 p-4 border-t border-slate-800 flex justify-between items-center text-xs text-slate-500 font-mono">
        <span>{{ record.id }}</span>
        {% if record.resolved_authors %}<span>Authors: {% for a in record.resolved_authors %}{{ a.initials }}{% if not loop.last %}, {% endif %}{% endfor %}</span>{% elif record.authors %}<span>Authors: {{ record.authors | join(", ") }}</span>{% endif %}
    </div>
</div>
{% endblock %}
"##;

const GRAPH_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}Graph - {{ site.title }}{% endblock %}

{% block head %}
<script src="https://d3js.org/d3.v7.min.js"></script>
{% endblock %}

{% block content %}
<div class="graph-container" id="graph"></div>
{% endblock %}

{% block scripts %}
<script>
const data = {{ graph_data | safe }};
const width = document.getElementById('graph').clientWidth;
const height = 500;

const color = d3.scaleOrdinal()
    .domain(['DEC', 'STR', 'POL', 'CUS', 'OPP', 'PRC', 'HIR', 'ADR', 'INC', 'RUN', 'MTG'])
    .range(['#4CAF50', '#2196F3', '#FF9800', '#9C27B0', '#E91E63', '#00BCD4', '#795548', '#607D8B', '#F44336', '#8BC34A', '#03A9F4']);

const svg = d3.select('#graph')
    .append('svg')
    .attr('width', width)
    .attr('height', height);

const simulation = d3.forceSimulation(data.nodes)
    .force('link', d3.forceLink(data.edges).id(d => d.id).distance(100))
    .force('charge', d3.forceManyBody().strength(-300))
    .force('center', d3.forceCenter(width / 2, height / 2));

const link = svg.append('g')
    .selectAll('line')
    .data(data.edges)
    .join('line')
    .attr('stroke', '#666')
    .attr('stroke-width', 1);

const node = svg.append('g')
    .selectAll('g')
    .data(data.nodes)
    .join('g')
    .call(d3.drag()
        .on('start', dragstarted)
        .on('drag', dragged)
        .on('end', dragended));

node.append('circle')
    .attr('r', 20)
    .attr('fill', d => color(d.type))
    .attr('stroke', d => d.foundational ? 'gold' : '#333')
    .attr('stroke-width', d => d.foundational ? 3 : 1);

node.append('text')
    .text(d => d.id)
    .attr('text-anchor', 'middle')
    .attr('dy', 4)
    .attr('fill', '#fff')
    .attr('font-size', '10px');

node.append('title').text(d => d.title);
node.on('click', (e, d) => window.location.href = '/records/' + d.id);
node.style('cursor', 'pointer');

simulation.on('tick', () => {
    link.attr('x1', d => d.source.x).attr('y1', d => d.source.y)
        .attr('x2', d => d.target.x).attr('y2', d => d.target.y);
    node.attr('transform', d => `translate(${d.x},${d.y})`);
});

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
const nodeRadius = 14;
const nodeSpacing = 50; // Minimum vertical spacing between nodes

// Assign lanes (columns) to avoid overlap - git-style
const lanes = {};  // type -> lane index
const typeOrder = ['DEC', 'STR', 'POL', 'ADR', 'INC', 'RUN', 'PRC', 'HIR', 'CUS', 'OPP', 'MTG'];
typeOrder.forEach((t, i) => lanes[t] = i);

// Get unique types in our data and assign lanes
const dataTypes = [...new Set(records.map(r => r.type))];
dataTypes.sort((a, b) => (typeOrder.indexOf(a) !== -1 ? typeOrder.indexOf(a) : 99) - (typeOrder.indexOf(b) !== -1 ? typeOrder.indexOf(b) : 99));
dataTypes.forEach((t, i) => lanes[t] = i);

const laneWidth = 70;
const width = margin.left + (dataTypes.length) * laneWidth + margin.right;

// Group records by year and calculate year heights based on record count
const recordsByYear = {};
years.forEach(y => recordsByYear[y] = []);
records.forEach(r => recordsByYear[r.year].push(r));

// Calculate year section heights (minimum 120px, grows with record count)
const yearHeights = {};
const minYearHeight = 120;
years.forEach(y => {
    const count = recordsByYear[y].length;
    yearHeights[y] = Math.max(minYearHeight, count * nodeSpacing + 40);
});

// Calculate cumulative Y offsets for each year
const yearOffsets = {};
let cumulativeY = margin.top;
years.forEach(y => {
    yearOffsets[y] = cumulativeY;
    cumulativeY += yearHeights[y];
});

const height = cumulativeY + margin.bottom;

// Calculate positions - spread nodes evenly within each year section
years.forEach(year => {
    const yearRecords = recordsByYear[year];
    if (yearRecords.length === 0) return;

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

// Draw year separators and labels
years.forEach((year, i) => {
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
    circle.setAttribute('stroke', r.foundational ? 'gold' : '#333');
    circle.setAttribute('stroke-width', r.foundational ? 3 : 1);
    g.appendChild(circle);

    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    text.setAttribute('text-anchor', 'middle');
    text.setAttribute('dy', 4);
    text.setAttribute('fill', '#fff');
    text.setAttribute('font-size', '9px');
    text.setAttribute('font-family', 'monospace');
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
<h2 class="text-2xl font-bold text-white mb-6">Statistics</h2>

<div class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-8">
    <div class="bg-piper-card border border-slate-700 rounded-xl p-6 text-center">
        <div class="text-4xl font-bold text-piper-light">{{ stats.total_records }}</div>
        <div class="text-slate-400 mt-1">Total Records</div>
    </div>
    <div class="bg-piper-card border border-slate-700 rounded-xl p-6 text-center">
        <div class="text-4xl font-bold text-piper-light">{{ stats.total_edges }}</div>
        <div class="text-slate-400 mt-1">Total Links</div>
    </div>
    <div class="bg-piper-card border border-slate-700 rounded-xl p-6 text-center">
        <div class="text-4xl font-bold text-yellow-500">{{ stats.foundational }}</div>
        <div class="text-slate-400 mt-1">Core Records</div>
    </div>
</div>

<h3 class="text-lg font-semibold text-white mb-4">By Type</h3>
<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3 mb-8">
    {% for item in stats.by_type %}
    <a href="/?type={{ item.type }}" class="bg-piper-card border border-slate-700 rounded-xl p-4 text-center hover:border-piper-light/50 hover:-translate-y-0.5 transition-all">
        <div class="text-2xl font-bold text-piper-light">{{ item.count }}</div>
        <div class="text-slate-400 text-sm">{{ item.type_display }}</div>
    </a>
    {% endfor %}
</div>

<h3 class="text-lg font-semibold text-white mb-4">By Status</h3>
<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
    {% for item in stats.by_status %}
    <a href="/?status={{ item.status }}" class="bg-piper-card border border-slate-700 rounded-xl p-4 text-center hover:border-piper-light/50 hover:-translate-y-0.5 transition-all">
        <div class="text-2xl font-bold text-piper-light">{{ item.count }}</div>
        <div class="text-slate-400 text-sm capitalize">{{ item.status }}</div>
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
        grid-template-columns: 1fr 1fr;
        gap: 1rem;
        flex: 1;
        min-height: 0;
    }
    .editor-layout.editor-only { grid-template-columns: 1fr; }
    .editor-layout.editor-only .preview-pane { display: none; }
    .editor-layout.preview-only { grid-template-columns: 1fr; }
    .editor-layout.preview-only .editor-pane-wrapper { display: none; }
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
        .editor-layout { grid-template-columns: 1fr; grid-template-rows: 1fr 1fr; }
        .editor-layout.editor-only, .editor-layout.preview-only { grid-template-rows: 1fr; }
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
        <div class="flex gap-3">
            <button id="saveBtn" class="px-4 py-2 bg-piper-accent hover:bg-piper-light text-white font-medium rounded-lg transition-all flex items-center gap-2">
                <svg id="saveIcon" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                <span id="saveBtnText">Save</span>
            </button>
            <a href="/records/{{ record_id }}" class="cancel-btn px-4 py-2 bg-slate-700 hover:bg-slate-600 text-white font-medium rounded-lg transition-colors">Cancel</a>
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
        <div id="metadataFields" class="p-4 grid grid-cols-1 md:grid-cols-4 gap-4">
            <div class="md:col-span-2">
                <label for="fieldTitle" class="block text-xs text-slate-500 mb-1">Title</label>
                <input type="text" id="fieldTitle" class="field-input" placeholder="Record title">
            </div>
            <div>
                <label for="fieldStatus" class="block text-xs text-slate-500 mb-1">Status</label>
                <select id="fieldStatus" class="field-input">
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
            <div class="flex items-end pb-1">
                <label for="fieldFoundational" class="flex items-center gap-2 text-sm text-slate-300 cursor-pointer select-none">
                    <input type="checkbox" id="fieldFoundational" class="rounded bg-slate-800 border-slate-600 text-piper-accent focus:ring-piper-accent focus:ring-offset-slate-900">
                    <span>Core</span>
                </label>
            </div>
            <div class="md:col-span-2">
                <label for="authorInput" class="block text-xs text-slate-500 mb-1">Authors</label>
                <div id="authorsContainer" class="field-input tag-input" onclick="document.getElementById('authorInput').focus()">
                    <input type="text" id="authorInput" class="bg-transparent border-none outline-none text-sm flex-1 min-w-[80px]" placeholder="Add author...">
                </div>
            </div>
            <div class="md:col-span-2">
                <label for="tagInput" class="block text-xs text-slate-500 mb-1">Tags</label>
                <div id="tagsContainer" class="field-input tag-input" onclick="document.getElementById('tagInput').focus()">
                    <input type="text" id="tagInput" class="bg-transparent border-none outline-none text-sm flex-1 min-w-[80px]" placeholder="Add tag...">
                </div>
            </div>
        </div>
    </div>

    <!-- Editor -->
    <div id="editorLayout" class="editor-layout">
        <div class="editor-pane-wrapper bg-piper-card border border-slate-700 rounded-xl overflow-hidden">
            <div class="px-4 py-2 bg-slate-800/50 border-b border-slate-700 flex justify-between items-center flex-shrink-0">
                <span class="text-xs font-mono uppercase tracking-wider text-slate-500">Content</span>
                <span id="cursorPos" class="text-xs text-slate-500 font-mono">Ln 1, Col 1</span>
            </div>
            <textarea id="editor" class="w-full p-4 bg-transparent text-slate-200 border-none outline-none" spellcheck="false" placeholder="Write your content here..."></textarea>
        </div>
        <div class="preview-pane bg-piper-card border border-slate-700 rounded-xl overflow-hidden">
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
    fieldFoundational.checked = fm.foundational || false;

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
        foundational: fieldFoundational.checked || undefined,
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
    if (fm.foundational) yaml += `foundational: true\n`;
    yaml += `links:\n`;
    for (const [k, v] of Object.entries(fm.links)) {
        yaml += `  ${k}: [${(v || []).join(', ')}]\n`;
    }
    yaml += '---\n\n';

    return yaml + editor.value;
}

function renderMarkdown(md) {
    let html = md
        .replace(/```(\w*)\n([\s\S]*?)```/g, '<pre class="bg-slate-800 p-4 rounded-lg overflow-x-auto my-4"><code>$2</code></pre>')
        .replace(/`([^`]+)`/g, '<code class="bg-slate-800 px-1.5 py-0.5 rounded text-piper-light">$1</code>')
        .replace(/^### (.+)$/gm, '<h3 class="text-lg font-semibold text-white mt-6 mb-2">$1</h3>')
        .replace(/^## (.+)$/gm, '<h2 class="text-xl font-bold text-white mt-8 mb-3 pb-2 border-b border-slate-700">$1</h2>')
        .replace(/^# (.+)$/gm, '<h1 class="text-2xl font-bold text-white mt-6 mb-4">$1</h1>')
        .replace(/\*\*([^*]+)\*\*/g, '<strong class="font-semibold text-white">$1</strong>')
        .replace(/\*([^*]+)\*/g, '<em>$1</em>')
        .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" class="text-piper-light hover:underline">$1</a>')
        .replace(/^- \[x\] (.+)$/gm, '<li class="ml-4 flex items-center gap-2"><input type="checkbox" checked disabled> <span class="line-through text-slate-500">$1</span></li>')
        .replace(/^- \[ \] (.+)$/gm, '<li class="ml-4 flex items-center gap-2"><input type="checkbox" disabled> $1</li>')
        .replace(/^- (.+)$/gm, '<li class="ml-4">$1</li>')
        .replace(/^> (.+)$/gm, '<blockquote class="border-l-4 border-piper-accent pl-4 my-4 text-slate-400 italic">$1</blockquote>')
        .replace(/\n\n/g, '</p><p class="my-4">')
        .replace(/\n/g, '<br>');
    html = '<p class="my-4">' + html + '</p>';
    html = html.replace(/(<li[\s\S]*?<\/li>)+/g, '<ul class="my-4">$&</ul>');
    return html;
}

function updatePreview() {
    preview.innerHTML = renderMarkdown(editor.value);
    document.getElementById('displayTitle').textContent = fieldTitle.value || 'Untitled';
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
    env
}
