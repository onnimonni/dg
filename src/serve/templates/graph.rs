pub const GRAPH_TEMPLATE: &str = r##"{% extends "base.html" %}

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
