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

<div class="flex gap-2 mb-6 flex-wrap">
    <button class="filter-btn px-4 py-2 border border-slate-700 rounded-lg bg-transparent text-slate-300 cursor-pointer hover:bg-slate-700 transition-colors active" data-type="all">All</button>
    {% for rt in record_types %}
    <button class="filter-btn px-4 py-2 border border-slate-700 rounded-lg bg-transparent text-slate-300 cursor-pointer hover:bg-slate-700 transition-colors" data-type="{{ rt.code }}">{{ rt.display }}</button>
    {% endfor %}
    <button id="sort" class="filter-btn px-4 py-2 border border-slate-700 rounded-lg bg-transparent text-slate-300 cursor-pointer hover:bg-slate-700 transition-colors ml-auto" title="Core First">★</button>
</div>

<div id="records" class="space-y-3">
{% for record in records %}
<a href="/records/{{ record.id }}" class="card block bg-piper-card border border-slate-700 rounded-xl p-4 hover:border-piper-light/50 hover:bg-slate-700/30 transition-all hover:-translate-y-0.5 {% if record.foundational %}border-l-4 border-l-yellow-500{% endif %}" data-type="{{ record.type }}" data-status="{{ record.status }}" data-id="{{ record.id }}" data-created="{{ record.created }}" data-foundational="{{ record.foundational }}">
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
        {% for tag in record.tags %}<span class="px-1.5 py-0.5 bg-slate-800 rounded text-xs text-slate-400 font-mono">#{{ tag }}</span>{% endfor %}
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
let activeType = 'all';
let activeStatus = 'all';
let sortMode = 'default'; // default -> newest -> oldest -> default

const sortModes = {
    default: { next: 'newest', icon: '★', title: 'Core First' },
    newest: { next: 'oldest', icon: '↓', title: 'Newest First' },
    oldest: { next: 'default', icon: '↑', title: 'Oldest First' }
};

// URL state management
function updateUrl() {
    const params = new URLSearchParams();
    if (search.value) params.set('q', search.value);
    if (activeType !== 'all') params.set('type', activeType);
    if (activeStatus !== 'all') params.set('status', activeStatus);
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
        r.style.display = matchesType && matchesStatus && matchesQuery ? 'block' : 'none';
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
                <div class="flex -space-x-2">
                    {% for author in record.resolved_authors %}
                    <div class="relative group">
                        <img src="{{ author.avatar_url }}" alt="{{ author.name }}" class="w-8 h-8 rounded-full border-2 border-slate-800 bg-piper-accent" title="{{ author.name }}">
                        <div class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-2 py-1 bg-slate-900 border border-slate-700 rounded text-xs text-slate-200 whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-10">
                            {{ author.name }}{% if author.email %} &lt;{{ author.email }}&gt;{% endif %}
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
                <span class="px-2 py-1 bg-slate-800 rounded text-xs text-slate-300 font-mono hover:bg-slate-700 cursor-pointer">#{{ tag }}</span>
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
const nodeRadius = 24;
const nodeSpacing = 60; // Minimum vertical spacing between nodes

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
    g.addEventListener('mouseenter', (e) => {
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
        tooltip.style.left = (e.clientX + 12) + 'px';
        tooltip.style.top = (e.clientY - 10) + 'px';
    });
    g.addEventListener('mousemove', (e) => {
        tooltip.style.left = (e.clientX + 12) + 'px';
        tooltip.style.top = (e.clientY - 10) + 'px';
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
    .editor-container {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1rem;
        height: calc(100vh - 200px);
        min-height: 500px;
    }
    .editor-pane {
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }
    .editor-pane textarea {
        flex: 1;
        resize: none;
        font-family: 'JetBrains Mono', monospace;
        font-size: 14px;
        line-height: 1.6;
        tab-size: 2;
    }
    .preview-pane {
        overflow-y: auto;
    }
    .preview-content {
        font-family: 'Inter', system-ui, sans-serif;
    }
    @media (max-width: 768px) {
        .editor-container {
            grid-template-columns: 1fr;
            grid-template-rows: 1fr 1fr;
        }
    }
    /* Syntax highlighting for frontmatter */
    .frontmatter-indicator {
        position: absolute;
        left: 0;
        top: 0;
        width: 4px;
        background: linear-gradient(to bottom, #007c43 0%, #007c43 var(--fm-height, 0%), transparent var(--fm-height, 0%));
        height: 100%;
        pointer-events: none;
    }
</style>
{% endblock %}

{% block content %}
<div class="w-full">
    <!-- Header -->
    <div class="flex justify-between items-center mb-6">
        <div class="flex items-center gap-4">
            <a href="/records/{{ record_id }}" class="p-2 hover:bg-slate-700 rounded-lg transition-colors text-slate-400 hover:text-white" title="Back to record">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path></svg>
            </a>
            <div>
                <h1 class="text-2xl font-bold text-white">Edit {{ record_id }}</h1>
                <p class="text-slate-400 text-sm">{{ record_title }}</p>
            </div>
        </div>
        <div class="flex gap-3">
            <button id="saveBtn" class="px-4 py-2 bg-piper-accent hover:bg-piper-light text-white font-medium rounded-lg transition-colors flex items-center gap-2">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                Save
            </button>
            <a href="/records/{{ record_id }}" class="px-4 py-2 bg-slate-700 hover:bg-slate-600 text-white font-medium rounded-lg transition-colors">
                Cancel
            </a>
        </div>
    </div>

    <!-- Status bar -->
    <div id="statusBar" class="mb-4 px-4 py-2 rounded-lg text-sm hidden"></div>

    <!-- Editor -->
    <div class="editor-container">
        <!-- Markdown Editor -->
        <div class="editor-pane bg-piper-card border border-slate-700 rounded-xl overflow-hidden">
            <div class="px-4 py-2 bg-slate-800/50 border-b border-slate-700 flex justify-between items-center">
                <span class="text-xs font-mono uppercase tracking-wider text-slate-500">Markdown</span>
                <span id="cursorPos" class="text-xs text-slate-500 font-mono">Ln 1, Col 1</span>
            </div>
            <div class="relative flex-1 flex">
                <textarea
                    id="editor"
                    class="w-full p-4 bg-transparent text-slate-200 border-none outline-none"
                    spellcheck="false"
                    placeholder="Enter markdown content..."
                >{{ raw_content }}</textarea>
            </div>
        </div>

        <!-- Preview -->
        <div class="editor-pane preview-pane bg-piper-card border border-slate-700 rounded-xl overflow-hidden">
            <div class="px-4 py-2 bg-slate-800/50 border-b border-slate-700">
                <span class="text-xs font-mono uppercase tracking-wider text-slate-500">Preview</span>
            </div>
            <div id="preview" class="preview-content p-4 text-slate-300 content"></div>
        </div>
    </div>

    <!-- Keyboard shortcuts help -->
    <div class="mt-4 text-xs text-slate-500 flex gap-6">
        <span><kbd class="px-1.5 py-0.5 bg-slate-800 rounded border border-slate-700">Ctrl</kbd> + <kbd class="px-1.5 py-0.5 bg-slate-800 rounded border border-slate-700">S</kbd> Save</span>
        <span><kbd class="px-1.5 py-0.5 bg-slate-800 rounded border border-slate-700">Esc</kbd> Cancel</span>
    </div>
</div>
{% endblock %}

{% block scripts %}
<script>
const editor = document.getElementById('editor');
const preview = document.getElementById('preview');
const saveBtn = document.getElementById('saveBtn');
const statusBar = document.getElementById('statusBar');
const cursorPos = document.getElementById('cursorPos');
const recordId = '{{ record_id }}';

let originalContent = editor.value;
let isDirty = false;

// Simple markdown to HTML converter (basic)
function renderMarkdown(md) {
    // Remove frontmatter for preview
    let content = md;
    if (content.startsWith('---')) {
        const endIndex = content.indexOf('---', 3);
        if (endIndex !== -1) {
            content = content.substring(endIndex + 3).trim();
        }
    }

    // Basic markdown rendering
    let html = content
        // Code blocks
        .replace(/```(\w*)\n([\s\S]*?)```/g, '<pre class="bg-slate-800 p-4 rounded-lg overflow-x-auto my-4"><code>$2</code></pre>')
        // Inline code
        .replace(/`([^`]+)`/g, '<code class="bg-slate-800 px-1.5 py-0.5 rounded text-piper-light">$1</code>')
        // Headers
        .replace(/^### (.+)$/gm, '<h3 class="text-lg font-semibold text-white mt-6 mb-2">$1</h3>')
        .replace(/^## (.+)$/gm, '<h2 class="text-xl font-bold text-white mt-8 mb-3 pb-2 border-b border-slate-700">$1</h2>')
        .replace(/^# (.+)$/gm, '<h1 class="text-2xl font-bold text-white mt-6 mb-4">$1</h1>')
        // Bold and italic
        .replace(/\*\*([^*]+)\*\*/g, '<strong class="font-semibold text-white">$1</strong>')
        .replace(/\*([^*]+)\*/g, '<em>$1</em>')
        // Links
        .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" class="text-piper-light hover:underline">$1</a>')
        // Lists
        .replace(/^- (.+)$/gm, '<li class="ml-4">$1</li>')
        .replace(/^(\d+)\. (.+)$/gm, '<li class="ml-4">$2</li>')
        // Task lists
        .replace(/^- \[x\] (.+)$/gm, '<li class="ml-4 flex items-center gap-2"><input type="checkbox" checked disabled class="rounded"> <span class="line-through text-slate-500">$1</span></li>')
        .replace(/^- \[ \] (.+)$/gm, '<li class="ml-4 flex items-center gap-2"><input type="checkbox" disabled class="rounded"> $1</li>')
        // Tables (basic)
        .replace(/^\|(.+)\|$/gm, (match, content) => {
            const cells = content.split('|').map(c => c.trim());
            if (cells.every(c => c.match(/^[-:]+$/))) {
                return ''; // Skip separator row
            }
            const cellHtml = cells.map(c => `<td class="border border-slate-700 px-3 py-2">${c}</td>`).join('');
            return `<tr>${cellHtml}</tr>`;
        })
        // Blockquotes
        .replace(/^> (.+)$/gm, '<blockquote class="border-l-4 border-piper-accent pl-4 my-4 text-slate-400 italic">$1</blockquote>')
        // Horizontal rules
        .replace(/^---$/gm, '<hr class="border-slate-700 my-6">')
        // Paragraphs
        .replace(/\n\n/g, '</p><p class="my-4">')
        .replace(/\n/g, '<br>');

    // Wrap in paragraph
    html = '<p class="my-4">' + html + '</p>';

    // Wrap tables
    html = html.replace(/(<tr>[\s\S]*?<\/tr>)+/g, '<table class="w-full border-collapse my-4">$&</table>');

    // Wrap lists
    html = html.replace(/(<li[\s\S]*?<\/li>)+/g, '<ul class="my-4">$&</ul>');

    return html;
}

function updatePreview() {
    preview.innerHTML = renderMarkdown(editor.value);
    isDirty = editor.value !== originalContent;
    updateTitle();
}

function updateTitle() {
    document.title = (isDirty ? '• ' : '') + 'Edit {{ record_id }} - {{ site.title }}';
}

function updateCursorPosition() {
    const text = editor.value.substring(0, editor.selectionStart);
    const lines = text.split('\n');
    const line = lines.length;
    const col = lines[lines.length - 1].length + 1;
    cursorPos.textContent = `Ln ${line}, Col ${col}`;
}

function showStatus(message, type = 'info') {
    statusBar.className = 'mb-4 px-4 py-2 rounded-lg text-sm ' + {
        'success': 'bg-green-900/50 border border-green-700 text-green-300',
        'error': 'bg-red-900/50 border border-red-700 text-red-300',
        'info': 'bg-blue-900/50 border border-blue-700 text-blue-300',
        'warning': 'bg-yellow-900/50 border border-yellow-700 text-yellow-300'
    }[type];
    statusBar.textContent = message;
    statusBar.classList.remove('hidden');

    if (type === 'success') {
        setTimeout(() => statusBar.classList.add('hidden'), 3000);
    }
}

async function save() {
    saveBtn.disabled = true;
    saveBtn.innerHTML = '<svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg> Saving...';

    try {
        const response = await fetch(`/api/records/${recordId}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ content: editor.value })
        });

        const data = await response.json();

        if (response.ok) {
            showStatus('Saved successfully!', 'success');
            originalContent = editor.value;
            isDirty = false;
            updateTitle();
        } else {
            showStatus(data.error || 'Failed to save', 'error');
        }
    } catch (err) {
        showStatus('Network error: ' + err.message, 'error');
    } finally {
        saveBtn.disabled = false;
        saveBtn.innerHTML = '<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg> Save';
    }
}

// Event listeners
editor.addEventListener('input', updatePreview);
editor.addEventListener('keyup', updateCursorPosition);
editor.addEventListener('click', updateCursorPosition);
saveBtn.addEventListener('click', save);

// Keyboard shortcuts
document.addEventListener('keydown', (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault();
        save();
    }
    if (e.key === 'Escape') {
        if (isDirty) {
            if (confirm('You have unsaved changes. Are you sure you want to leave?')) {
                window.location.href = '/records/' + recordId;
            }
        } else {
            window.location.href = '/records/' + recordId;
        }
    }
});

// Tab key support
editor.addEventListener('keydown', (e) => {
    if (e.key === 'Tab') {
        e.preventDefault();
        const start = editor.selectionStart;
        const end = editor.selectionEnd;
        editor.value = editor.value.substring(0, start) + '  ' + editor.value.substring(end);
        editor.selectionStart = editor.selectionEnd = start + 2;
        updatePreview();
    }
});

// Warn before leaving with unsaved changes
window.addEventListener('beforeunload', (e) => {
    if (isDirty) {
        e.preventDefault();
        e.returnValue = '';
    }
});

// Initial render
updatePreview();
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
