use minijinja::Environment;

const BASE_TEMPLATE: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{% block title %}{{ site.title }}{% endblock %}</title>
    {% if site.description %}<meta name="description" content="{{ site.description }}">{% endif %}
    <style>
        :root {
            --bg: #1a1a2e;
            --surface: #16213e;
            --primary: {{ site.primary_color | default(value="#0f3460") }};
            --accent: {{ site.accent_color | default(value="#e94560") }};
            --text: #e2e8f0;
            --text-dim: #94a3b8;
            --success: #4CAF50;
            --warning: #FF9800;
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.6;
        }
        header {
            background: var(--surface);
            padding: 1rem 2rem;
            border-bottom: 1px solid var(--primary);
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        header .brand { display: flex; align-items: center; gap: 0.75rem; text-decoration: none; color: inherit; }
        header .brand img { height: 32px; }
        header h1 { font-size: 1.5rem; }
        header nav a {
            color: var(--text);
            text-decoration: none;
            margin-left: 1.5rem;
            opacity: 0.7;
            padding: 0.25rem 0.5rem;
            border-radius: 4px;
        }
        header nav a:hover { opacity: 1; }
        header nav a.active { opacity: 1; background: var(--primary); }
        main { max-width: 1200px; margin: 2rem auto; padding: 0 2rem; }
        footer {
            text-align: center;
            padding: 2rem;
            color: var(--text-dim);
            font-size: 0.9rem;
        }
        .card {
            background: var(--surface);
            border-radius: 8px;
            padding: 1.5rem;
            margin-bottom: 1rem;
            border-left: 4px solid var(--primary);
        }
        .card[data-type="DEC"] { border-left-color: #4CAF50; }
        .card[data-type="ADR"] { border-left-color: #607D8B; }
        .card[data-type="INC"] { border-left-color: #F44336; }
        .card[data-type="POL"] { border-left-color: #FF9800; }
        .card[data-type="RUN"] { border-left-color: #8BC34A; }
        .card[data-type="STR"] { border-left-color: #2196F3; }
        .card[data-type="PRC"] { border-left-color: #00BCD4; }
        .card[data-type="CUS"] { border-left-color: #9C27B0; }
        .card[data-type="OPP"] { border-left-color: #E91E63; }
        .card[data-type="HIR"] { border-left-color: #795548; }
        .card[data-type="MTG"] { border-left-color: #03A9F4; }
        .card.foundational { border-left-color: gold; }
        .card-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 0.5rem;
        }
        .card-id {
            font-family: monospace;
            color: var(--accent);
            font-weight: bold;
        }
        .card-title { font-size: 1.2rem; color: var(--text); text-decoration: none; display: block; }
        .card-title:hover { color: #fff; }
        .card-meta { color: var(--text-dim); font-size: 0.85rem; margin-top: 0.35rem; }
        .detail-card .card-title { font-size: 1.75rem; font-weight: 700; color: #fff; }
        .detail-card .card-meta { padding-bottom: 1rem; border-bottom: 1px solid rgba(148,163,184,0.15); }
        .badge {
            display: inline-block;
            padding: 0.25rem 0.5rem;
            border-radius: 4px;
            font-size: 0.8rem;
            background: var(--primary);
        }
        .badge.accepted { background: var(--success); }
        .badge.proposed { background: var(--warning); color: #000; }
        .badge.open { background: #e74c3c; }
        .badge.resolved { background: #3498db; }
        .badge.draft { background: #666; }
        .badge.deprecated { background: #95a5a6; }
        .badge.superseded { background: #7f8c8d; }
        .tag {
            display: inline-block;
            padding: 0.15rem 0.5rem;
            border-radius: 3px;
            font-size: 0.75rem;
            font-family: monospace;
            background: rgba(148,163,184,0.1);
            color: var(--text-dim);
            margin-right: 0.25rem;
        }
        .links { margin-top: 1.5rem; padding-top: 1.5rem; border-top: 1px solid rgba(148,163,184,0.15); }
        .links-title { font-size: 0.7rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.1em; color: var(--text-dim); font-family: monospace; margin-bottom: 0.75rem; }
        .link-card { display: block; padding: 0.75rem 1rem; background: rgba(148,163,184,0.06); border: 1px solid rgba(148,163,184,0.1); border-radius: 8px; margin-bottom: 0.5rem; text-decoration: none; transition: all 0.15s; }
        .link-card:hover { background: rgba(148,163,184,0.12); border-color: rgba(148,163,184,0.2); transform: translateY(-1px); text-decoration: none; }
        .link-card .link-id { font-family: monospace; font-size: 0.8rem; font-weight: 600; }
        .link-card .link-rel { font-size: 0.7rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-dim); border: 1px solid rgba(148,163,184,0.2); padding: 0.1rem 0.4rem; border-radius: 3px; float: right; }
        .link-type { color: var(--text-dim); font-size: 0.9rem; }
        a { color: var(--accent); text-decoration: none; }
        a:hover { text-decoration: underline; }
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
        }
        .stat-card {
            background: var(--surface);
            padding: 1.5rem;
            border-radius: 8px;
            text-align: center;
        }
        .stat-value { font-size: 2rem; font-weight: bold; color: var(--accent); }
        .stat-label { color: var(--text-dim); }
        .graph-container { background: var(--surface); border-radius: 8px; padding: 1rem; min-height: 500px; }
        .content { background: var(--surface); border-radius: 8px; padding: 2rem; margin-top: 1rem; max-width: 48rem; line-height: 1.75; }
        .content h1 { font-size: 1.5rem; font-weight: 700; margin-top: 2rem; margin-bottom: 0.75rem; color: var(--text); }
        .content h2 { font-size: 1.25rem; font-weight: 600; margin-top: 1.75rem; margin-bottom: 0.5rem; color: var(--text); }
        .content h3 { font-size: 1.1rem; font-weight: 600; margin-top: 1.5rem; margin-bottom: 0.5rem; color: var(--text); }
        .content h1:first-child, .content h2:first-child { margin-top: 0; }
        .content p { margin-bottom: 1rem; }
        .content ul, .content ol { margin-left: 1.5rem; margin-bottom: 1rem; }
        .content li { margin-bottom: 0.35rem; }
        .content strong { font-weight: bold; }
        .content em { font-style: italic; }
        .content code { background: var(--primary); padding: 0.2rem 0.4rem; border-radius: 3px; font-family: monospace; }
        .content pre { background: var(--primary); padding: 1rem; border-radius: 8px; overflow-x: auto; margin-bottom: 1rem; }
        .content pre code { background: none; padding: 0; }
        .content blockquote { border-left: 3px solid var(--accent); padding-left: 1rem; margin: 1rem 0; color: var(--text-dim); }
        .content table { width: 100%; border-collapse: collapse; margin-bottom: 1rem; }
        .content th, .content td { padding: 0.5rem; border: 1px solid var(--primary); text-align: left; }
        .content th { background: var(--primary); }
        .record-link {
            display: inline;
            padding: 0.1rem 0.35rem;
            background: rgba(148, 163, 184, 0.15);
            border-radius: 3px;
            font-family: monospace;
            font-size: 0.85em;
            color: #94a3b8;
            text-decoration: none;
            cursor: pointer;
            position: relative;
            vertical-align: baseline;
        }
        .record-link:hover { background: rgba(148, 163, 184, 0.25); text-decoration: underline; color: #e2e8f0; }
        .record-preview {
            position: absolute;
            bottom: 100%;
            left: 50%;
            transform: translateX(-50%);
            background: var(--surface);
            border: 1px solid var(--primary);
            border-radius: 8px;
            padding: 0.75rem 1rem;
            min-width: 280px;
            max-width: 350px;
            z-index: 1000;
            box-shadow: 0 4px 12px rgba(0,0,0,0.4);
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.15s;
            margin-bottom: 8px;
            text-align: left;
        }
        .record-link:hover .record-preview { opacity: 1; }
        .record-preview::after {
            content: '';
            position: absolute;
            top: 100%;
            left: 50%;
            transform: translateX(-50%);
            border: 8px solid transparent;
            border-top-color: var(--primary);
        }
        .preview-title { font-weight: bold; margin-bottom: 0.25rem; color: var(--text); font-family: inherit; }
        .preview-meta { font-size: 0.8rem; color: var(--text-dim); font-family: inherit; }
        .preview-status { display: inline-block; padding: 0.1rem 0.3rem; border-radius: 3px; font-size: 0.75rem; margin-left: 0.5rem; }
        .preview-status.accepted { background: var(--success); }
        .preview-status.proposed { background: var(--warning); color: #000; }
        .preview-status.resolved { background: #3498db; }
        .search-box { width: 100%; padding: 0.75rem; border: 1px solid var(--primary); border-radius: 8px; background: var(--surface); color: var(--text); margin-bottom: 1.5rem; }
        .filter-bar { display: flex; gap: 1rem; margin-bottom: 1.5rem; flex-wrap: wrap; }
        .filter-btn { padding: 0.5rem 1rem; border: 1px solid var(--primary); border-radius: 4px; background: transparent; color: var(--text); cursor: pointer; }
        .filter-btn.active { background: var(--primary); }
        {{ site.custom_css | default(value="") | safe }}
    </style>
    <link rel="stylesheet" href="/static/katex.min.css">
    <script defer src="/static/katex.min.js"></script>
    <script defer src="/static/auto-render.min.js" onload="renderMathInElement(document.body, {delimiters: [{left: '$$', right: '$$', display: true}, {left: '$', right: '$', display: false}]});"></script>
    {% block head %}{% endblock %}
</head>
<body>
    <header>
        <a href="/" class="brand">
            {% if site.logo %}<img src="{{ site.logo }}" alt="{{ site.title }}">
            {% else %}<h1>{{ site.title }}</h1>{% endif %}
        </a>
        <nav>
            <a href="/"{% if current_page == "records" %} class="active"{% endif %}>Records</a>
            <a href="/timeline"{% if current_page == "timeline" %} class="active"{% endif %}>Timeline</a>
            <a href="/graph"{% if current_page == "graph" %} class="active"{% endif %}>Graph</a>
            <a href="/stats"{% if current_page == "stats" %} class="active"{% endif %}>Stats</a>
        </nav>
    </header>
    <main>
        {% block content %}{% endblock %}
    </main>
    {% if site.footer %}
    <footer>{{ site.footer }} ©</footer>
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

{% block content %}
<input type="text" class="search-box" placeholder="Search records..." id="search">

<div class="filter-bar">
    <button class="filter-btn active" data-type="all">All</button>
    {% for rt in record_types %}
    <button class="filter-btn" data-type="{{ rt.code }}">{{ rt.display }}</button>
    {% endfor %}
    <button id="sort" class="filter-btn" style="margin-left: auto;" title="Core First">&#9733;</button>
</div>

<div id="records">
{% for record in records %}
<div class="card {% if record.foundational %}foundational{% endif %}" data-type="{{ record.type }}" data-id="{{ record.id }}" data-created="{{ record.created }}" data-foundational="{{ record.foundational }}">
    <div class="card-header">
        <div>
            <a href="/records/{{ record.id }}" class="card-id">{{ record.id }}</a>
            {% if record.foundational %}<span class="badge" style="background: gold; color: #000;">CORE</span>{% endif %}
        </div>
        <span class="badge {{ record.status }}">{{ record.status }}</span>
    </div>
    <a href="/records/{{ record.id }}" class="card-title">{{ record.title }}</a>
    <div class="card-meta">
        {{ record.type_display }} | {{ record.created }}
        {% if record.tags %}
        | {% for tag in record.tags %}<span class="tag">{{ tag }}</span>{% endfor %}
        {% endif %}
    </div>
</div>
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
        const matchesQuery = !query || r.textContent.toLowerCase().includes(query);
        r.style.display = matchesType && matchesQuery ? 'block' : 'none';
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
<nav style="margin-bottom: 1rem; font-size: 0.9rem; color: var(--text-dim);">
    <a href="/">Records</a> <span style="margin: 0 0.35rem;">›</span> <span>{{ record.id }}</span>
</nav>
<div class="card detail-card {% if record.foundational %}foundational{% endif %}" data-type="{{ record.type }}">
    <div class="card-header">
        <div>
            <span class="card-id">{{ record.id }}</span>
            {% if record.foundational %}<span class="badge" style="background: gold; color: #000;">FOUNDATIONAL</span>{% endif %}
        </div>
        <span class="badge {{ record.status }}">{{ record.status }}</span>
    </div>
    <h2 class="card-title">{{ record.title }}</h2>
    <div class="card-meta">
        {{ record.type_display }} · {{ record.created }}
        {% if record.authors %} · {{ record.authors | join(", ") }}{% endif %}
    </div>
    {% if record.tags %}
    <div style="margin-top: 0.75rem;">
        {% for tag in record.tags %}<span class="tag">{{ tag }}</span>{% endfor %}
    </div>
    {% endif %}

    {% if record.links %}
    <div class="links">
        <div class="links-title">Connections</div>
        {% for link in record.links %}
        <a href="/records/{{ link.target }}" class="link-card">
            <span class="link-rel">{{ link.type }}</span>
            <span class="link-id">{{ link.target }}</span>
        </a>
        {% endfor %}
    </div>
    {% endif %}
</div>

<div class="content">
    {{ record.content_html | safe }}
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
        transition: transform 0.15s;
    }
    .timeline-node:hover {
        transform: scale(1.1);
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
const yearHeight = 180;
const nodeRadius = 24;
const height = margin.top + years.length * yearHeight + margin.bottom;

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

// Calculate positions
records.forEach(r => {
    const yearIndex = r.year - minYear;
    const dayOfYear = (r.date - new Date(r.year, 0, 1)) / (1000 * 60 * 60 * 24);
    const yearProgress = dayOfYear / 365;
    r.y = margin.top + yearIndex * yearHeight + yearProgress * (yearHeight - 40) + 20;
    r.x = margin.left + lanes[r.type] * laneWidth + laneWidth / 2;
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
    const y = margin.top + i * yearHeight;

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
    text.setAttribute('y', y + yearHeight / 2);
    text.setAttribute('class', 'year-label');
    text.textContent = year;
    svg.appendChild(text);
});

// Draw trunk lines for each lane (vertical lines like git branches)
dataTypes.forEach((type, i) => {
    const x = margin.left + i * laneWidth + laneWidth / 2;
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
<h2 style="margin-bottom: 1.5rem;">Statistics</h2>

<div class="stats-grid">
    <div class="stat-card">
        <div class="stat-value">{{ stats.total_records }}</div>
        <div class="stat-label">Total Records</div>
    </div>
    <div class="stat-card">
        <div class="stat-value">{{ stats.total_edges }}</div>
        <div class="stat-label">Total Links</div>
    </div>
    <div class="stat-card">
        <div class="stat-value">{{ stats.foundational }}</div>
        <div class="stat-label">Foundational</div>
    </div>
</div>

<h3 style="margin: 2rem 0 1rem;">By Type</h3>
<div class="stats-grid">
    {% for item in stats.by_type %}
    <div class="stat-card">
        <div class="stat-value">{{ item.count }}</div>
        <div class="stat-label">{{ item.type_display }}</div>
    </div>
    {% endfor %}
</div>

<h3 style="margin: 2rem 0 1rem;">By Status</h3>
<div class="stats-grid">
    {% for item in stats.by_status %}
    <div class="stat-card">
        <div class="stat-value">{{ item.count }}</div>
        <div class="stat-label">{{ item.status }}</div>
    </div>
    {% endfor %}
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
    env
}
