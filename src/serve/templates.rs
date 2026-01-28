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
            --text: #eee;
            --text-dim: #999;
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
        .card-title { font-size: 1.2rem; }
        .card-meta { color: var(--text-dim); font-size: 0.9rem; }
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
            padding: 0.1rem 0.4rem;
            border-radius: 3px;
            font-size: 0.75rem;
            background: var(--primary);
            margin-right: 0.25rem;
        }
        .links { margin-top: 1rem; padding-top: 1rem; border-top: 1px solid var(--primary); }
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
        .content { background: var(--surface); border-radius: 8px; padding: 2rem; margin-top: 1rem; }
        .content h1, .content h2, .content h3 { margin-top: 1.5rem; margin-bottom: 0.5rem; color: var(--text); }
        .content h1:first-child, .content h2:first-child { margin-top: 0; }
        .content p { margin-bottom: 1rem; }
        .content ul, .content ol { margin-left: 1.5rem; margin-bottom: 1rem; }
        .content li { margin-bottom: 0.25rem; }
        .content strong { font-weight: bold; }
        .content em { font-style: italic; }
        .content code { background: var(--primary); padding: 0.2rem 0.4rem; border-radius: 3px; font-family: monospace; }
        .content pre { background: var(--primary); padding: 1rem; border-radius: 8px; overflow-x: auto; margin-bottom: 1rem; }
        .content pre code { background: none; padding: 0; }
        .content blockquote { border-left: 3px solid var(--accent); padding-left: 1rem; margin: 1rem 0; color: var(--text-dim); }
        .content table { width: 100%; border-collapse: collapse; margin-bottom: 1rem; }
        .content th, .content td { padding: 0.5rem; border: 1px solid var(--primary); text-align: left; }
        .content th { background: var(--primary); }
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
            <a href="/graph"{% if current_page == "graph" %} class="active"{% endif %}>Graph</a>
            <a href="/stats"{% if current_page == "stats" %} class="active"{% endif %}>Stats</a>
        </nav>
    </header>
    <main>
        {% block content %}{% endblock %}
    </main>
    {% if site.footer %}
    <footer>{{ site.footer }}</footer>
    {% endif %}
    {% block scripts %}{% endblock %}
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

search.addEventListener('input', filterRecords);
sortBtn.addEventListener('click', cycleSortMode);
filters.forEach(btn => {
    if (btn.id !== 'sort' && btn.tagName === 'BUTTON') {
        btn.addEventListener('click', () => {
            filters.forEach(b => { if (b.id !== 'sort' && b.tagName === 'BUTTON') b.classList.remove('active'); });
            btn.classList.add('active');
            activeType = btn.dataset.type;
            filterRecords();
        });
    }
});

function cycleSortMode() {
    sortMode = sortModes[sortMode].next;
    sortBtn.innerHTML = sortModes[sortMode].icon;
    sortBtn.title = sortModes[sortMode].title;
    sortRecords();
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
</script>
{% endblock %}
"##;

const RECORD_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}{{ record.id }} - {{ site.title }}{% endblock %}

{% block content %}
<div class="card {% if record.foundational %}foundational{% endif %}">
    <div class="card-header">
        <div>
            <span class="card-id">{{ record.id }}</span>
            {% if record.foundational %}<span class="badge" style="background: gold; color: #000;">FOUNDATIONAL</span>{% endif %}
        </div>
        <span class="badge {{ record.status }}">{{ record.status }}</span>
    </div>
    <h2 class="card-title">{{ record.title }}</h2>
    <div class="card-meta">
        {{ record.type_display }} | Created: {{ record.created }} | Updated: {{ record.updated }}
        {% if record.authors %} | Authors: {{ record.authors | join(", ") }}{% endif %}
    </div>
    {% if record.tags %}
    <div style="margin-top: 0.5rem;">
        {% for tag in record.tags %}<span class="tag">{{ tag }}</span>{% endfor %}
    </div>
    {% endif %}

    {% if record.links %}
    <div class="links">
        {% for link in record.links %}
        <div class="link-type">
            {{ link.type }}: <a href="/records/{{ link.target }}">{{ link.target }}</a>
        </div>
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
    env
}
