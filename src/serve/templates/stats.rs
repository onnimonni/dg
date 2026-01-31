pub const STATS_TEMPLATE: &str = r##"{% extends "base.html" %}

{% block title %}Stats - {{ site.title }}{% endblock %}

{% block content %}
<h1 class="text-3xl font-bold mb-6">Statistics</h1>

<div class="stats stats-vertical sm:stats-horizontal shadow w-full mb-8">
    <div class="stat">
        <div class="stat-title">Total Records</div>
        <div class="stat-value text-primary">{{ stats.total_records }}</div>
    </div>
    <div class="stat">
        <div class="stat-title">Total Links</div>
        <div class="stat-value text-secondary">{{ stats.total_edges }}</div>
    </div>
    <div class="stat">
        <div class="stat-title">Core Records</div>
        <div class="stat-value text-warning">{{ stats.core }}</div>
    </div>
</div>

<h3 class="text-lg font-semibold mb-4">By Type</h3>
<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3 mb-8">
    {% for item in stats.by_type %}
    <a href="/?type={{ item.type }}" class="card card-border bg-base-100 hover:bg-base-200 hover:-translate-y-0.5 transition-all">
        <div class="card-body p-4 items-center text-center">
            <div class="text-2xl font-bold text-primary">{{ item.count }}</div>
            <div class="text-sm opacity-60">{{ item.type_display }}</div>
        </div>
    </a>
    {% endfor %}
</div>

<h3 class="text-lg font-semibold mb-4">By Status</h3>
<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
    {% for item in stats.by_status %}
    <a href="/?status={{ item.status }}" class="card card-border bg-base-100 hover:bg-base-200 hover:-translate-y-0.5 transition-all">
        <div class="card-body p-4 items-center text-center">
            <div class="text-2xl font-bold text-primary">{{ item.count }}</div>
            <div class="text-sm opacity-60 capitalize">{{ item.status }}</div>
        </div>
    </a>
    {% endfor %}
</div>
{% endblock %}
"##;
