pub const TEAMS_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}Teams - {{ site.title }}{% endblock %}

{% block content %}
<div class="flex flex-col md:flex-row justify-between items-end mb-6 border-b border-slate-700 pb-4 gap-4">
    <div>
        <h2 class="text-2xl font-bold text-white mb-1">Organization Chart</h2>
        <p class="text-sm text-slate-400">Teams, members, and stakeholders</p>
    </div>
    <div class="flex gap-2 flex-wrap items-center">
        <span class="inline-flex items-center rounded-full bg-slate-800 border border-slate-700 px-2.5 py-1 text-xs font-medium text-slate-400">{{ active_user_count }} Active</span>
        {% if current_user_team %}
        <span class="inline-flex items-center rounded-full bg-piper-accent/20 border border-piper-accent/50 px-2.5 py-1 text-xs font-medium text-piper-light">Your Team: {{ current_user_team }}</span>
        {% endif %}
    </div>
</div>

<div class="flex flex-col xl:flex-row gap-6">
    <!-- Main Area: Core Teams -->
    <div class="flex-1 min-w-0">
        {% if core_teams | length > 0 %}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {% for team in core_teams %}
            <div class="bg-piper-card border border-slate-700 rounded-xl overflow-hidden shadow-lg hover:shadow-xl hover:border-slate-600 transition-all {% if team.is_current_user_team %}ring-2 ring-piper-accent ring-offset-2 ring-offset-slate-900{% endif %}">
                <!-- Team header with colored border -->
                <div class="h-1 bg-gradient-to-r from-piper-accent to-emerald-400"></div>
                <div class="p-4">
                    <div class="flex items-center justify-between mb-3">
                        <a href="/teams/{{ team.id }}" class="flex items-center gap-3 group">
                            <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-piper-accent to-emerald-400 flex items-center justify-center text-white font-bold shadow-md">
                                {{ team.name | first | upper }}
                            </div>
                            <div>
                                <h3 class="font-medium text-white group-hover:text-piper-light transition-colors">{{ team.name }}</h3>
                                <div class="text-xs text-slate-500">{{ team.member_count }} members</div>
                            </div>
                        </a>
                        {% if team.is_current_user_team %}
                        <span class="inline-flex items-center rounded-full bg-piper-accent/20 border border-piper-accent/50 px-2 py-0.5 text-[10px] font-medium text-piper-light">Your Team</span>
                        {% endif %}
                    </div>

                    {% if team.description %}
                    <p class="text-sm text-slate-400 mb-3">{{ team.description }}</p>
                    {% endif %}

                    <!-- Team Lead -->
                    {% if team.lead %}
                    <div class="mb-3">
                        <div class="text-[10px] uppercase tracking-wider text-slate-500 mb-1.5 font-medium">Lead</div>
                        <a href="/users/{{ team.lead }}" class="flex items-center gap-2.5 p-2 rounded-lg bg-slate-800/50 hover:bg-slate-800 transition-colors group">
                            <div class="relative">
                                <div class="w-9 h-9 rounded-full {% for m in team.members %}{% if m.username == team.lead %}{{ m.avatar_color }}{% endif %}{% endfor %} flex items-center justify-center text-sm font-medium text-white {% if team.lead_is_current %}ring-2 ring-piper-accent ring-offset-1 ring-offset-slate-800{% endif %}">
                                    {% for m in team.members %}{% if m.username == team.lead %}{{ m.initials }}{% endif %}{% endfor %}
                                </div>
                                {% if team.lead_is_current %}
                                <span class="absolute -top-1 -right-1 px-1 text-[9px] rounded bg-piper-accent text-white font-medium">You</span>
                                {% endif %}
                            </div>
                            <div class="flex-1 min-w-0">
                                <div class="font-medium text-sm text-white group-hover:text-piper-light transition-colors truncate">
                                    {% for m in team.members %}{% if m.username == team.lead %}{{ m.name }}{% endif %}{% endfor %}
                                </div>
                                <div class="text-xs text-slate-500">@{{ team.lead }}</div>
                            </div>
                        </a>
                    </div>
                    {% endif %}

                    <!-- Team Members -->
                    {% if team.members | length > 0 %}
                    <div class="text-[10px] uppercase tracking-wider text-slate-500 mb-1.5 font-medium">Members</div>
                    <div class="flex flex-wrap gap-1.5">
                        {% for member in team.members %}
                        {% if member.username != team.lead %}
                        <a href="/users/{{ member.username }}" class="relative group/member" title="{{ member.name }}">
                            <div class="w-8 h-8 rounded-full {{ member.avatar_color }} flex items-center justify-center text-xs font-medium text-white hover:ring-2 hover:ring-slate-500 transition-all {% if member.is_current_user %}ring-2 ring-piper-accent{% endif %}">
                                {{ member.initials }}
                            </div>
                            {% if member.is_current_user %}
                            <span class="absolute -top-1 -right-1 px-1 text-[9px] rounded bg-piper-accent text-white font-medium">You</span>
                            {% endif %}
                            <span class="absolute -bottom-7 left-1/2 -translate-x-1/2 px-2 py-1 text-xs bg-slate-900 border border-slate-700 rounded whitespace-nowrap opacity-0 group-hover/member:opacity-100 transition-opacity pointer-events-none z-10 shadow-lg">{{ member.name }}</span>
                        </a>
                        {% endif %}
                        {% endfor %}
                    </div>
                    {% endif %}
                </div>
            </div>
            {% endfor %}
        </div>
        {% endif %}

        <!-- Other Teams -->
        {% if other_teams | length > 0 %}
        <div class="mt-6">
            <h3 class="text-sm font-medium text-white mb-3">Other Teams</h3>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                {% for team in other_teams %}
                <a href="/teams/{{ team.id }}" class="flex items-center gap-3 p-3 bg-piper-card border border-slate-700 rounded-lg shadow hover:shadow-md hover:border-slate-600 transition-all">
                    <div class="w-9 h-9 rounded-lg bg-slate-700 flex items-center justify-center text-white font-bold text-sm">
                        {{ team.name | first | upper }}
                    </div>
                    <div class="flex-1 min-w-0">
                        <div class="font-medium text-sm text-white truncate">{{ team.name }}</div>
                        <div class="text-xs text-slate-500">{{ team.member_count }} members</div>
                    </div>
                </a>
                {% endfor %}
            </div>
        </div>
        {% endif %}

        <!-- Mobile: Stakeholders accordion (hidden on xl) -->
        <div class="xl:hidden mt-6">
            <button onclick="document.getElementById('mobile-stakeholders').classList.toggle('hidden'); this.querySelector('.chevron').classList.toggle('rotate-180');" class="w-full flex items-center justify-between p-3 bg-slate-800/50 border border-slate-700 rounded-lg text-sm font-medium text-white hover:bg-slate-800 transition-colors">
                <span>Stakeholders & External</span>
                <svg class="chevron w-5 h-5 text-slate-400 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
            </button>
            <div id="mobile-stakeholders" class="hidden mt-3 p-4 bg-slate-800/30 border border-slate-700 rounded-lg space-y-4">
                {% for team in stakeholder_teams %}
                {% if team.members | length > 0 %}
                <div>
                    <div class="text-[10px] font-bold tracking-widest text-slate-500 uppercase mb-2">{{ team.name }}</div>
                    <div class="space-y-1.5">
                        {% for member in team.members %}
                        <a href="/users/{{ member.username }}" class="flex items-center gap-2.5 p-2 rounded-lg hover:bg-slate-800/50 transition-colors">
                            <div class="w-7 h-7 rounded-full {{ member.avatar_color }} flex items-center justify-center text-xs font-medium text-white">{{ member.initials }}</div>
                            <div class="flex-1 min-w-0">
                                <div class="font-medium text-sm text-white truncate">{{ member.name }}</div>
                                {% if member.hashtag_teams | length > 0 %}
                                <div class="flex flex-wrap gap-1">
                                    {% for ht in member.hashtag_teams %}
                                    <span class="inline-flex items-center rounded-full bg-slate-800 border border-slate-700 px-1.5 py-0.5 text-[9px] font-medium text-slate-400">#{{ ht.id }}</span>
                                    {% endfor %}
                                </div>
                                {% endif %}
                            </div>
                        </a>
                        {% endfor %}
                    </div>
                </div>
                {% endif %}
                {% endfor %}
            </div>
        </div>
    </div>

    <!-- Sidebar: Stakeholders (hidden on mobile, visible xl+) -->
    <div class="hidden xl:block w-72 flex-shrink-0 border-l border-slate-700 pl-6 bg-slate-900/30 -mr-4 pr-4 py-2 rounded-r-lg">
        <div class="text-xs font-bold tracking-widest text-slate-400 uppercase mb-4">Stakeholders</div>
        {% for team in stakeholder_teams %}
        {% if team.members | length > 0 %}
        <div class="mb-5">
            <div class="text-[10px] font-bold tracking-widest text-slate-500 uppercase mb-2">{{ team.name }}</div>
            <div class="space-y-1.5">
                {% for member in team.members %}
                <a href="/users/{{ member.username }}" class="flex items-center gap-2.5 p-2 rounded-lg bg-slate-800/30 border border-slate-700/50 hover:bg-slate-800/70 hover:border-slate-600 transition-all group">
                    <div class="relative">
                        <div class="w-7 h-7 rounded-full {{ member.avatar_color }} flex items-center justify-center text-xs font-medium text-white transition-colors {% if member.is_current_user %}ring-2 ring-piper-accent{% endif %}">
                            {{ member.initials }}
                        </div>
                        {% if member.is_current_user %}
                        <span class="absolute -top-1 -right-1 px-1 text-[8px] rounded bg-piper-accent text-white font-medium">You</span>
                        {% endif %}
                    </div>
                    <div class="flex-1 min-w-0">
                        <div class="font-medium text-sm text-white truncate">{{ member.name }}</div>
                        {% if member.roles | length > 0 %}
                        <div class="text-[10px] text-slate-500 truncate">{{ member.roles | first }}</div>
                        {% endif %}
                        {% if member.hashtag_teams | length > 0 %}
                        <div class="flex flex-wrap gap-1 mt-0.5">
                            {% for ht in member.hashtag_teams %}
                            <span class="inline-flex items-center rounded-full bg-slate-800 border border-slate-700 px-1.5 py-0.5 text-[9px] font-medium text-slate-400">#{{ ht.id }}</span>
                            {% endfor %}
                        </div>
                        {% endif %}
                    </div>
                </a>
                {% endfor %}
            </div>
        </div>
        {% endif %}
        {% endfor %}

        <!-- Deprecated Users toggle -->
        {% if deprecated_users | length > 0 %}
        <div class="mt-4 pt-4 border-t border-slate-700">
            <button onclick="var el = document.getElementById('deprecated-users'); var track = document.getElementById('deprecated-track'); var knob = document.getElementById('deprecated-knob'); el.classList.toggle('hidden'); if(el.classList.contains('hidden')) { track.classList.add('bg-slate-700'); track.classList.remove('bg-piper-accent'); knob.classList.add('translate-x-0'); knob.classList.remove('translate-x-4'); } else { track.classList.remove('bg-slate-700'); track.classList.add('bg-piper-accent'); knob.classList.remove('translate-x-0'); knob.classList.add('translate-x-4'); }" class="w-full flex items-center justify-between group">
                <span class="text-[10px] font-bold tracking-widest text-slate-500 uppercase group-hover:text-slate-400 transition-colors">Deprecated ({{ deprecated_users | length }})</span>
                <span class="relative inline-flex h-4 w-7 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent bg-slate-700 transition-colors duration-200 ease-in-out" id="deprecated-track">
                    <span class="translate-x-0 pointer-events-none inline-block h-3 w-3 transform rounded-full bg-slate-400 shadow ring-0 transition duration-200 ease-in-out" id="deprecated-knob"></span>
                </span>
            </button>
            <div id="deprecated-users" class="hidden mt-3 space-y-1.5">
                {% for user in deprecated_users %}
                <a href="/users/{{ user.username }}" class="flex items-center gap-2.5 p-2 rounded-lg bg-slate-800/20 opacity-60 hover:opacity-100 transition-all">
                    <div class="w-7 h-7 rounded-full bg-slate-800 flex items-center justify-center text-xs font-medium text-slate-500">
                        {{ user.initials }}
                    </div>
                    <div class="flex-1 min-w-0">
                        <div class="font-medium text-sm text-slate-400 line-through truncate">{{ user.name }}</div>
                        <span class="inline-flex items-center rounded-full bg-red-900/30 border border-red-500/30 px-1.5 py-0.5 text-[9px] font-medium text-red-400">Left</span>
                    </div>
                </a>
                {% endfor %}
            </div>
        </div>
        {% endif %}
    </div>
</div>

{% if core_teams | length == 0 and stakeholder_teams | length == 0 and other_teams | length == 0 %}
<div class="text-center text-slate-500 py-12">
    No teams configured. Add teams to <code class="bg-slate-800 px-2 py-1 rounded">dg.toml</code>
</div>
{% endif %}
{% endblock %}
"##;

pub const TEAM_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}{{ team.name }} - {{ site.title }}{% endblock %}

{% block content %}
<div class="max-w-3xl mx-auto">
    <div class="bg-piper-card border border-slate-700 rounded-2xl overflow-hidden">
        <div class="h-1.5 w-full bg-gradient-to-r from-piper-accent to-emerald-400"></div>

        <div class="p-8">
            <div class="flex items-start gap-6">
                <div class="w-24 h-24 rounded-xl bg-gradient-to-br from-piper-accent to-emerald-400 flex items-center justify-center text-white font-bold text-3xl">
                    {{ team.name | first | upper }}
                </div>
                <div class="flex-1">
                    <h1 class="text-3xl font-bold text-white mb-2">{{ team.name }}</h1>
                    <div class="text-lg text-slate-400 mb-4">{{ team.id }}</div>

                    {% if team.description %}
                    <p class="text-slate-300 mb-4">{{ team.description }}</p>
                    {% endif %}

                    {% if team.lead %}
                    <div class="flex items-center gap-2 text-sm text-slate-400 mb-2">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/></svg>
                        Lead: <a href="/users/{{ team.lead }}" class="text-piper-light hover:underline">@{{ team.lead }}</a>
                    </div>
                    {% endif %}

                    {% if team.email %}
                    <div class="flex items-center gap-2 text-sm text-slate-400 mb-2">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>
                        {{ team.email }}
                    </div>
                    {% endif %}

                    {% if team.parent %}
                    <div class="flex items-center gap-2 text-sm text-slate-400">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/></svg>
                        Parent: <a href="/teams/{{ team.parent }}" class="text-piper-light hover:underline">{{ team.parent }}</a>
                    </div>
                    {% endif %}

                    <div class="mt-4">
                        <a href="/teams/{{ team.id }}/history" class="inline-flex items-center gap-2 text-sm text-piper-light hover:text-piper-accent transition-colors">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                            View History
                        </a>
                    </div>
                </div>
            </div>
        </div>
    </div>

    {% if members %}
    <div class="mt-8">
        <h3 class="text-lg font-semibold text-white mb-4">Members ({{ members | length }})</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            {% for member in members %}
            <a href="/users/{{ member.username }}" class="flex items-center gap-3 bg-piper-card border border-slate-700 rounded-xl p-3 hover:border-piper-light/50 transition-all">
                <img src="{{ member.avatar_url }}" alt="{{ member.name }}" class="w-10 h-10 rounded-full border-2 border-slate-700">
                <div>
                    <div class="text-white font-medium">{{ member.name }}</div>
                    <div class="text-sm text-slate-400">@{{ member.username }}</div>
                </div>
            </a>
            {% endfor %}
        </div>
    </div>
    {% endif %}

    {% if sub_teams %}
    <div class="mt-8">
        <h3 class="text-lg font-semibold text-white mb-4">Sub-teams ({{ sub_teams | length }})</h3>
        <div class="space-y-3">
            {% for sub in sub_teams %}
            <a href="/teams/{{ sub.id }}" class="block bg-piper-card border border-slate-700 rounded-xl p-4 hover:border-piper-light/50 transition-all">
                <div class="font-semibold text-white">{{ sub.name }}</div>
                {% if sub.lead %}
                <div class="text-sm text-slate-400">Lead: @{{ sub.lead }}</div>
                {% endif %}
            </a>
            {% endfor %}
        </div>
    </div>
    {% endif %}
</div>
{% endblock %}
"##;

pub const TEAM_HISTORY_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}{{ team_name }} History - {{ site.title }}{% endblock %}

{% block content %}
<div class="max-w-4xl mx-auto">
    <div class="mb-6">
        <a href="/teams/{{ team_id }}" class="text-piper-light hover:underline text-sm">← Back to {{ team_name }}</a>
    </div>

    <div class="bg-piper-card border border-slate-700 rounded-2xl overflow-hidden">
        <div class="h-1.5 w-full bg-gradient-to-r from-piper-accent to-emerald-400"></div>

        <div class="p-8">
            <h1 class="text-2xl font-bold text-white mb-2">{{ team_name }} History</h1>
            <p class="text-slate-400 mb-6">Team membership changes over time from git history</p>

            {% if history %}
            <div class="space-y-6">
                {% for snapshot in history %}
                <div class="relative pl-6 border-l-2 border-slate-700 {% if loop.first %}border-l-emerald-500{% endif %}">
                    <div class="absolute -left-2 top-0 w-4 h-4 rounded-full {% if loop.first %}bg-emerald-500{% else %}bg-slate-600{% endif %}"></div>

                    <div class="mb-2">
                        <span class="text-slate-300 font-medium">{{ snapshot.date }}</span>
                        <span class="text-slate-600 mx-2">·</span>
                        <span class="font-mono text-xs text-slate-500">{{ snapshot.commit }}</span>
                    </div>

                    <p class="text-sm text-slate-400 mb-3">{{ snapshot.message }}</p>

                    {% if snapshot.joined %}
                    <div class="flex flex-wrap gap-2 mb-2">
                        {% for user in snapshot.joined %}
                        <span class="inline-flex items-center rounded-md bg-green-500/10 px-2 py-1 text-xs font-medium text-green-400 ring-1 ring-inset ring-green-500/20">
                            + @{{ user }}
                        </span>
                        {% endfor %}
                    </div>
                    {% endif %}

                    {% if snapshot.left %}
                    <div class="flex flex-wrap gap-2 mb-2">
                        {% for user in snapshot.left %}
                        <span class="inline-flex items-center rounded-md bg-red-500/10 px-2 py-1 text-xs font-medium text-red-400 ring-1 ring-inset ring-red-500/20">
                            − @{{ user }}
                        </span>
                        {% endfor %}
                    </div>
                    {% endif %}

                    <div class="text-xs text-slate-500">
                        Members: {% for user in snapshot.members %}<a href="/users/{{ user }}" class="text-slate-400 hover:text-piper-light">@{{ user }}</a>{% if not loop.last %}, {% endif %}{% endfor %}
                    </div>
                </div>
                {% endfor %}
            </div>
            {% else %}
            <div class="text-center py-12 text-slate-500">
                <p>No history found for this team.</p>
                <p class="text-sm mt-2">Team membership changes are tracked when dg.toml is committed to git.</p>
            </div>
            {% endif %}
        </div>
    </div>

    {% if all_time_members %}
    <div class="mt-8 bg-piper-card border border-slate-700 rounded-xl p-6">
        <h3 class="text-lg font-semibold text-white mb-4">All-Time Members ({{ all_time_members | length }})</h3>
        <div class="flex flex-wrap gap-2">
            {% for user in all_time_members %}
            <a href="/users/{{ user }}" class="inline-flex items-center rounded-md bg-slate-700/50 px-3 py-1.5 text-sm text-slate-300 hover:bg-slate-600 transition-colors">
                @{{ user }}
            </a>
            {% endfor %}
        </div>
    </div>
    {% endif %}
</div>
{% endblock %}
"##;
