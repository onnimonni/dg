pub const INDEX_TEMPLATE: &str = r##"{% extends "base.html" %}

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
<a href="/records/{{ record.id }}" class="card card-border bg-base-100 hover:bg-base-200 transition-all hover:-translate-y-0.5 flex flex-col {% if record.core %}border-l-4 border-l-warning{% endif %}" data-type="{{ record.type }}" data-status="{{ record.status }}" data-id="{{ record.id }}" data-created="{{ record.created }}" data-core="{{ record.core }}" data-tags="{{ record.tags | join(',') }}">
    <div class="card-body p-5 flex flex-col flex-1">
        <div class="flex justify-between items-center gap-3 mb-1">
            <div class="flex items-center gap-2">
                <span class="font-mono text-xs opacity-40">{{ record.id }}</span>
                {% if record.is_draft %}<span class="badge badge-xs badge-secondary badge-outline">DRAFT</span>{% endif %}
                {% if record.core %}<span class="badge badge-xs badge-warning badge-outline">CORE</span>{% endif %}
            </div>
            <span class="badge badge-sm flex-shrink-0 {% if record.status == 'accepted' or record.status == 'active' %}badge-success{% elif record.status == 'proposed' or record.status == 'draft' %}badge-warning{% elif record.status == 'open' %}badge-error{% elif record.status == 'rejected' %}badge-error badge-outline{% elif record.status == 'resolved' %}badge-info{% elif record.status == 'deprecated' %}badge-warning badge-outline{% elif record.status == 'superseded' %}badge-neutral{% else %}badge-neutral{% endif %}">{{ record.status | upper }}</span>
        </div>
        <h3 class="text-base font-semibold text-base-content flex-1">{{ record.title }}</h3>
        <div class="mt-auto pt-3">
            <div class="flex flex-wrap items-center gap-x-2 gap-y-1 text-xs opacity-50 mb-2">
                <span class="badge badge-xs badge-ghost">{{ record.type_display }}</span>
                {% if record.status == 'deprecated' %}<span class="text-warning">{{ record.created_month_year }} → {{ record.updated_month_year }}{% if record.duration %} <span class="opacity-70">({{ record.duration }})</span>{% endif %}</span>{% elif record.type == 'INC' and record.status == 'open' %}<span class="text-error">{{ record.created_month_year }} → ongoing{% if record.duration %} <span class="opacity-70">({{ record.duration }})</span>{% endif %}</span>{% elif record.type == 'INC' and record.status == 'resolved' %}<span class="text-info">{{ record.created_month_year }} → {{ record.updated_month_year }}{% if record.duration %} <span class="opacity-70">({{ record.duration }})</span>{% endif %}</span>{% elif record.type == 'CUS' and record.created_year != record.updated_year %}<span>{{ record.created_month_year }} → {{ record.updated_month_year }}{% if record.duration %} <span class="opacity-70">({{ record.duration }})</span>{% endif %}</span>{% else %}<span>{{ record.created_month_year }}</span>{% endif %}
            </div>
            {% if record.tags %}
            <div class="flex flex-wrap gap-1">{% for tag in record.tags %}<span class="tag-link badge badge-xs badge-outline opacity-60 cursor-pointer hover:opacity-100" data-tag="{{ tag }}">#{{ tag }}</span>{% endfor %}</div>
            {% endif %}
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
                    <span class="badge badge-sm {% if record.status == 'accepted' or record.status == 'active' %}badge-success{% elif record.status == 'proposed' or record.status == 'draft' %}badge-warning{% elif record.status == 'open' %}badge-error{% elif record.status == 'rejected' %}badge-error badge-outline{% elif record.status == 'resolved' %}badge-info{% elif record.status == 'deprecated' %}badge-warning badge-outline{% elif record.status == 'superseded' %}badge-neutral{% else %}badge-neutral{% endif %}">{{ record.status }}</span>
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
