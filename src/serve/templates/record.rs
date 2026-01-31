pub const RECORD_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}{{ record.id }} - {{ site.title }}{% endblock %}

{% block content %}
<div class="w-full bg-piper-card border border-slate-700 rounded-2xl shadow-2xl overflow-hidden{% if record.is_inactive %} opacity-80{% endif %}">
    <!-- Accent bar - changes color based on status -->
    {% if record.status == 'deprecated' %}
    <div class="h-1.5 w-full bg-gradient-to-r from-amber-500 to-amber-600"></div>
    {% elif record.status == 'rejected' %}
    <div class="h-1.5 w-full bg-gradient-to-r from-red-500 to-red-600"></div>
    {% elif record.status == 'superseded' %}
    <div class="h-1.5 w-full bg-gradient-to-r from-slate-500 to-slate-600"></div>
    {% else %}
    <div class="h-1.5 w-full bg-gradient-to-r from-piper-accent to-emerald-400"></div>
    {% endif %}

    <!-- Warning banner for non-active documents -->
    {% if record.status == 'deprecated' %}
    <div class="bg-amber-900/30 border-b border-amber-800/40 px-6 py-4">
        <div class="flex items-start gap-3">
            <svg class="w-5 h-5 text-amber-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
            </svg>
            <div>
                <p class="text-amber-200 font-semibold">This document is deprecated</p>
                <p class="text-amber-300/70 text-sm mt-1">This decision or record is no longer active. The information may be outdated.</p>
            </div>
        </div>
    </div>
    {% elif record.status == 'rejected' %}
    <div class="bg-red-900/30 border-b border-red-800/40 px-6 py-4">
        <div class="flex items-start gap-3">
            <svg class="w-5 h-5 text-red-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
            <div>
                <p class="text-red-200 font-semibold">This proposal was rejected</p>
                <p class="text-red-300/70 text-sm mt-1">This decision was not approved. Review the rationale below for context.</p>
            </div>
        </div>
    </div>
    {% elif record.status == 'superseded' %}
    <div class="bg-slate-700/50 border-b border-slate-600/40 px-6 py-4">
        <div class="flex items-start gap-3">
            <svg class="w-5 h-5 text-slate-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"/>
            </svg>
            <div>
                <p class="text-slate-200 font-semibold">This document has been superseded</p>
                <p class="text-slate-400 text-sm mt-1">
                    A newer version of this decision exists.
                    {% if record.superseded_by and record.superseded_by | length > 0 %}
                    See: {% for link in record.superseded_by %}<a href="/records/{{ link.id }}" class="text-piper-light hover:underline">{{ link.id }}{% if link.title %}: {{ link.title }}{% endif %}</a>{% if not loop.last %}, {% endif %}{% endfor %}
                    {% endif %}
                </p>
            </div>
        </div>
    </div>
    {% endif %}

    <div class="p-8 pb-4">
        <!-- Header row -->
        <div class="flex justify-between items-start mb-4">
            <div class="flex items-center gap-3">
                <span class="font-mono text-xs text-slate-500">{{ record.id }}</span>
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
                <div class="avatar-group -space-x-6">
                    {% for author in record.resolved_authors %}
                    <div class="avatar author-wrapper">
                        <a href="/users/{{ author.username }}" class="w-8">
                            <img src="{{ author.avatar_url }}" alt="{{ author.name }}" class="author-avatar bg-piper-accent rounded-full" data-initials="{{ author.initials }}">
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
        <div class="mt-6 pr-4">
            <nav id="toc" class="hidden lg:block w-56 mb-4 pl-8 border-l border-slate-700/50" style="float: right; margin-left: 3rem;">
                <div class="sticky top-6">
                    <button id="toc-toggle" class="flex items-center gap-1.5 text-xs font-bold text-slate-500 uppercase tracking-widest mb-4 font-mono hover:text-slate-400 transition-colors">
                        <svg id="toc-chevron" class="w-2.5 h-2.5 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M19 9l-7 7-7-7"></path></svg>
                        On This Page
                    </button>
                    <ul id="toc-list" class="list-none space-y-1 text-sm"></ul>
                </div>
            </nav>
            <div class="text-slate-300 leading-relaxed content" id="content">
                {{ record.content_html | safe }}
            </div>
            <div style="clear: both;"></div>
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
        <a href="#" onclick="window.scrollTo({top:0,behavior:'smooth'});return false;" class="hover:text-slate-300 transition-colors cursor-pointer">{{ record.id }}</a>
        {% if record.resolved_authors %}<span>Authors: {% for a in record.resolved_authors %}<a href="/users/{{ a.username }}" class="hover:text-slate-300 transition-colors">{{ a.name }}</a>{% if not loop.last %}, {% endif %}{% endfor %}</span>{% elif record.authors %}<span>Authors: {{ record.authors | join(", ") }}</span>{% endif %}
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
    const usedIds = new Set();
    headings.forEach((heading, index) => {
        // Add ID to heading if it doesn't have one - use human-readable slug from heading text
        if (!heading.id) {
            let slug = heading.textContent.toLowerCase()
                .replace(/[^a-z0-9\s-]/g, '')  // Remove special chars
                .replace(/\s+/g, '-')           // Spaces to dashes
                .replace(/-+/g, '-')            // Multiple dashes to single
                .replace(/^-|-$/g, '');         // Trim dashes from ends
            // Handle duplicates by adding index
            if (usedIds.has(slug)) {
                slug = slug + '-' + index;
            }
            usedIds.add(slug);
            heading.id = slug || 'heading-' + index;
        }

        // Make heading clickable to update URL (for sharing)
        heading.style.cursor = 'pointer';
        heading.title = 'Click to copy link';
        heading.addEventListener('click', () => {
            history.pushState(null, '', '#' + heading.id);
            // Copy URL to clipboard
            navigator.clipboard.writeText(window.location.href).then(() => {
                heading.title = 'Link copied!';
                setTimeout(() => heading.title = 'Click to copy link', 2000);
            });
        });

        const li = document.createElement('li');
        const a = document.createElement('a');
        a.href = '#' + heading.id;
        a.textContent = heading.textContent;
        a.className = 'block py-1.5 pl-4 text-slate-500 hover:text-slate-300 transition-colors border-l-2 border-slate-700/50';
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
                item.link.classList.remove('text-slate-500', 'border-slate-700/50');
                item.link.classList.add('text-white', 'border-piper-accent', 'border-l-3', 'font-medium');
            } else {
                item.link.classList.add('text-slate-500', 'border-slate-700/50');
                item.link.classList.remove('text-white', 'border-piper-accent', 'border-l-3', 'font-medium');
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
        // Collapse width when closed to give content more room
        toc.style.width = tocCollapsed ? 'auto' : '';
        toc.style.marginLeft = tocCollapsed ? '1rem' : '';
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
