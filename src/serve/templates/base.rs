pub const BASE_TEMPLATE: &str = r##"<!DOCTYPE html>
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
        /* Avatar group - overlapping avatars */
        .avatar-group {
            display: flex;
            align-items: center;
        }
        .avatar-group .avatar {
            --avatar-size: 2rem;
            position: relative;
            overflow: visible;
        }
        .avatar-group .avatar:hover {
            z-index: 10;
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
            transition: border-color 0.2s;
        }
        .author-wrapper:hover .author-avatar,
        .author-wrapper:hover .avatar-initials {
            border-color: var(--accent);
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
        /* Record ID hover preview tooltip */
        .record-link { position: relative; }
        .record-preview {
            position: absolute;
            top: 100%;
            left: 0;
            margin-top: 0.25rem;
            padding: 0.5rem 0.75rem;
            background: #0f172a;
            border: 1px solid #334155;
            border-radius: 0.5rem;
            font-size: 0.75rem;
            white-space: nowrap;
            z-index: 50;
            box-shadow: 0 4px 12px rgba(0,0,0,0.4);
        }
        .preview-title {
            color: #e2e8f0;
            font-weight: 500;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }
        .preview-status {
            font-size: 0.625rem;
            padding: 0.125rem 0.375rem;
            border-radius: 0.25rem;
            text-transform: uppercase;
            font-weight: 600;
        }
        .preview-status.accepted, .preview-status.active, .preview-status.resolved { background: #166534; color: #86efac; }
        .preview-status.proposed, .preview-status.draft { background: #854d0e; color: #fde047; }
        .preview-status.open { background: #991b1b; color: #fca5a5; }
        .preview-status.superseded, .preview-status.deprecated { background: #374151; color: #9ca3af; }
        .preview-meta {
            color: #94a3b8;
            font-size: 0.6875rem;
            margin-top: 0.25rem;
        }
        /* List styles for markdown content only */
        .content ul { list-style-type: disc; padding-left: 1.5rem; margin: 1rem 0; }
        .content ol { list-style-type: decimal; padding-left: 1.5rem; margin: 1rem 0; }
        .content li { margin: 0.25rem 0; }
        /* Reduce gap between headings and tables that immediately follow.
           Tables need to clear the floating TOC, and headings preceding tables
           should clear too so they stay visually connected. */
        .content h2:has(+ table),
        .content h3:has(+ table) { margin-bottom: 0.5rem; clear: right; }
        .content table { clear: right; margin-top: 0; }
        /* Mermaid diagram containers */
        .mermaid-container { background: #1e293b; border-radius: 0.5rem; padding: 1rem; overflow-x: auto; }
        .mermaid-container svg { max-width: 100%; height: auto; }
        {{ site.custom_css | default(value="") | safe }}
    </style>
    <script defer src="/static/katex.min.js"></script>
    <script defer src="/static/auto-render.min.js" onload="renderMathInElement(document.body, {delimiters: [{left: '$$', right: '$$', display: true}, {left: '$', right: '$', display: false}]});"></script>
    <link rel="stylesheet" href="/static/highlight-github-dark.min.css">
    <script defer src="/static/highlight.min.js" onload="hljs.highlightAll();"></script>
    <script type="module">
        import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs';
        mermaid.initialize({ startOnLoad: false, theme: 'dark' });
        document.addEventListener('DOMContentLoaded', () => {
            document.querySelectorAll('pre code.language-mermaid').forEach((el, i) => {
                const code = el.textContent;
                const container = document.createElement('div');
                container.className = 'mermaid-container my-4';
                el.parentElement.replaceWith(container);
                mermaid.render('mermaid-' + i, code).then(({svg}) => {
                    container.innerHTML = svg;
                }).catch(err => {
                    container.innerHTML = '<pre class="text-error">' + err + '</pre>';
                });
            });
        });
    </script>
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
