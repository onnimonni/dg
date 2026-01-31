pub const TIMELINE_TEMPLATE: &str = r##"{% extends "base.html" %}

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
        tooltip.innerHTML = `<strong>${r.id}</strong>: ${r.title}<br><span style="color:var(--text-dim)">${typeNames[r.type] || r.type} | ${r.created}</span>`;
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
