pub const USERS_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}Users - {{ site.title }}{% endblock %}

{% block content %}
<!-- Header with search and toggle -->
<div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 mb-6">
    <h2 class="text-2xl font-bold text-white">Users</h2>
    <div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-3 w-full sm:w-auto">
        <!-- Search input -->
        <div class="relative flex-1 sm:flex-initial">
            <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
            </svg>
            <input type="text" id="userSearch" placeholder="Search users..." class="w-full sm:w-64 pl-10 pr-4 py-2 bg-slate-800 border border-slate-700 rounded-lg text-sm text-white placeholder-slate-500 focus:outline-none focus:border-piper-accent focus:ring-1 focus:ring-piper-accent">
        </div>
        <!-- Toggle switch for deprecated -->
        <button type="button" id="showDeprecatedToggle" role="switch" aria-checked="false" class="group flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-slate-800/50 transition-colors">
            <span class="relative inline-flex h-5 w-9 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent bg-slate-700 transition-colors duration-200 ease-in-out" id="toggleTrack">
                <span class="translate-x-0 pointer-events-none inline-block h-4 w-4 transform rounded-full bg-slate-400 shadow ring-0 transition duration-200 ease-in-out" id="toggleKnob"></span>
            </span>
            <span class="text-sm text-slate-400 group-hover:text-slate-300">Show deprecated</span>
        </button>
    </div>
</div>

<!-- User grid - tighter layout -->
<div id="users" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3">
{% for user in users %}
<a href="/users/{{ user.username }}" class="user-card flex items-center gap-3 p-3 bg-piper-card border border-slate-700 rounded-lg hover:border-piper-light/50 hover:bg-slate-800/50 transition-all{% if user.is_deprecated %} opacity-50{% endif %}" data-deprecated="{{ user.is_deprecated }}" data-name="{{ user.name | lower }}" data-username="{{ user.username | lower }}">
    <!-- Avatar with deterministic color -->
    <div class="w-10 h-10 rounded-full {{ user.avatar_color }} flex items-center justify-center text-white text-sm font-medium flex-shrink-0">
        {{ user.initials }}
    </div>
    <div class="flex-1 min-w-0">
        <div class="flex items-center gap-1.5">
            <span class="font-medium text-sm text-white truncate">{{ user.name }}</span>
            {% if user.is_llm %}
            <span class="text-purple-400 text-xs">🤖</span>
            {% endif %}
            {% if user.is_deprecated %}
            <span class="inline-flex items-center rounded-full bg-slate-700 px-1.5 py-0.5 text-[10px] font-medium text-slate-400">LEFT</span>
            {% endif %}
        </div>
        <div class="text-xs text-slate-500">@{{ user.username }}</div>
        {% if user.teams %}
        <div class="flex flex-wrap gap-1 mt-1">
            {% for team in user.teams %}
            <span class="inline-flex items-center rounded-full bg-slate-800 border border-slate-700 px-2 py-0.5 text-[10px] font-medium text-slate-400">{{ team }}</span>
            {% endfor %}
        </div>
        {% endif %}
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
// Toggle switch functionality
const toggleBtn = document.getElementById('showDeprecatedToggle');
const toggleTrack = document.getElementById('toggleTrack');
const toggleKnob = document.getElementById('toggleKnob');
let showDeprecated = false;

toggleBtn.addEventListener('click', () => {
    showDeprecated = !showDeprecated;
    toggleBtn.setAttribute('aria-checked', showDeprecated);
    if (showDeprecated) {
        toggleTrack.classList.remove('bg-slate-700');
        toggleTrack.classList.add('bg-piper-accent');
        toggleKnob.classList.remove('translate-x-0', 'bg-slate-400');
        toggleKnob.classList.add('translate-x-4', 'bg-white');
    } else {
        toggleTrack.classList.add('bg-slate-700');
        toggleTrack.classList.remove('bg-piper-accent');
        toggleKnob.classList.add('translate-x-0', 'bg-slate-400');
        toggleKnob.classList.remove('translate-x-4', 'bg-white');
    }
    filterUsers();
});

// Search functionality
const searchInput = document.getElementById('userSearch');
searchInput.addEventListener('input', filterUsers);

function filterUsers() {
    const query = searchInput.value.toLowerCase();
    document.querySelectorAll('.user-card').forEach(card => {
        const isDeprecated = card.dataset.deprecated === 'true';
        const name = card.dataset.name || '';
        const username = card.dataset.username || '';
        const matchesSearch = name.includes(query) || username.includes(query);
        const shouldShow = matchesSearch && (showDeprecated || !isDeprecated);
        card.style.display = shouldShow ? 'flex' : 'none';
    });
}

// Hide deprecated by default
filterUsers();
</script>
{% endblock %}
"##;

pub const USER_TEMPLATE: &str = r##"{% extends "base.html" %}

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

    {% if user_records %}
    <div class="mt-8">
        <h3 class="text-lg font-semibold text-white mb-4">Records ({{ user_records | length }})</h3>
        <div class="overflow-x-auto">
            <table class="w-full text-sm">
                <thead>
                    <tr class="border-b border-slate-700 text-left text-slate-400 text-xs uppercase tracking-wider">
                        <th class="py-3 px-2 font-medium">ID</th>
                        <th class="py-3 px-2 font-medium">Title</th>
                        <th class="py-3 px-2 font-medium">Date</th>
                        <th class="py-3 px-2 font-medium text-center">Role</th>
                        <th class="py-3 px-2 font-medium text-right">Status</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-slate-800">
                    {% for record in user_records %}
                    <tr class="hover:bg-slate-800/50 transition-colors cursor-pointer {% if record.core %}bg-warning/5{% endif %}" onclick="window.location='/records/{{ record.id }}'">
                        <td class="py-3 px-2 font-mono text-piper-light whitespace-nowrap">{{ record.id }}{% if record.core %}<span class="ml-1 text-warning">★</span>{% endif %}</td>
                        <td class="py-3 px-2 text-slate-300">{{ record.title }}</td>
                        <td class="py-3 px-2 text-slate-500 whitespace-nowrap">{{ record.date }}</td>
                        <td class="py-3 px-2 text-center whitespace-nowrap">
                            {% if record.is_author %}
                            <span class="px-2 py-0.5 rounded text-xs font-medium bg-piper-accent/20 text-piper-light">Author</span>
                            {% endif %}
                            {% if record.daci_role %}
                            <span class="px-2 py-0.5 rounded text-xs font-medium
                                {% if record.daci_role == 'responsible' %}bg-red-900/30 text-red-400
                                {% elif record.daci_role == 'approver' %}bg-amber-900/30 text-amber-400
                                {% elif record.daci_role == 'consulted' %}bg-blue-900/30 text-blue-400
                                {% else %}bg-slate-700 text-slate-400{% endif %}">{{ record.daci_role | capitalize }}</span>
                            {% endif %}
                        </td>
                        <td class="py-3 px-2 text-right">
                            <span class="px-2 py-0.5 rounded text-xs font-semibold uppercase
                                {% if record.status == 'accepted' %}bg-green-900/30 text-green-500
                                {% elif record.status == 'resolved' %}bg-blue-900/30 text-blue-400
                                {% elif record.status == 'deprecated' %}bg-slate-700 text-slate-500
                                {% elif record.status == 'superseded' %}bg-slate-700 text-slate-500
                                {% else %}bg-slate-700 text-slate-400{% endif %}">{{ record.status }}</span>
                        </td>
                    </tr>
                    {% endfor %}
                </tbody>
            </table>
        </div>
    </div>
    {% endif %}

    {% if mentioned_in %}
    <div class="mt-8">
        <h3 class="text-lg font-semibold text-white mb-4">Mentioned In ({{ mentioned_in | length }})</h3>
        <div class="overflow-x-auto">
            <table class="w-full text-sm">
                <tbody class="divide-y divide-slate-800">
                    {% for record in mentioned_in %}
                    <tr class="hover:bg-slate-800/50 transition-colors cursor-pointer" onclick="window.location='/records/{{ record.id }}'">
                        <td class="py-3 px-2 font-mono text-amber-400 whitespace-nowrap">{{ record.id }}</td>
                        <td class="py-3 px-2 text-slate-300">{{ record.title }}</td>
                        <td class="py-3 px-2 text-right">
                            <span class="px-2 py-0.5 rounded text-xs font-semibold uppercase bg-slate-700 text-slate-400">{{ record.status }}</span>
                        </td>
                    </tr>
                    {% endfor %}
                </tbody>
            </table>
        </div>
    </div>
    {% endif %}

    {% if action_items %}
    {% set incomplete_count = action_items | selectattr("completed", "false") | list | length %}
    <div class="mt-8">
        <div class="flex items-center gap-3 mb-4">
            <h3 class="text-lg font-semibold text-white">Action Items</h3>
            {% if incomplete_count > 0 %}
            <span class="px-2 py-0.5 rounded-full text-xs font-medium bg-amber-900/50 text-amber-400">{{ incomplete_count }} open</span>
            {% else %}
            <span class="px-2 py-0.5 rounded-full text-xs font-medium bg-green-900/50 text-green-400">All done</span>
            {% endif %}
        </div>
        <div class="overflow-x-auto">
            <table class="w-full text-sm">
                <tbody class="divide-y divide-slate-800">
                    {% for item in action_items %}
                    <tr class="hover:bg-slate-800/50 transition-colors cursor-pointer" onclick="window.location='/records/{{ item.record_id }}'">
                        <td class="py-3 px-2 w-6">
                            {% if item.completed %}
                            <span class="text-green-500">✓</span>
                            {% else %}
                            <span class="text-amber-400">○</span>
                            {% endif %}
                        </td>
                        <td class="py-3 px-2 {% if item.completed %}text-slate-500 line-through{% else %}text-slate-300{% endif %}">
                            {% set clean_text = item.text %}
                            {% if item.owner %}{% set clean_text = item.text | replace("@" ~ item.owner, "") | trim %}{% endif %}
                            {{ clean_text }}
                            {% if item.owner and item.owner | lower != user.username | lower %}
                            <span class="ml-2 text-xs text-slate-500">via @{{ item.owner }}</span>
                            {% endif %}
                        </td>
                        <td class="py-3 px-2 text-right whitespace-nowrap">
                            <span class="font-mono text-xs text-slate-500">{{ item.record_id }}</span>
                        </td>
                    </tr>
                    {% endfor %}
                </tbody>
            </table>
        </div>
    </div>
    {% endif %}
</div>
{% endblock %}
"##;
