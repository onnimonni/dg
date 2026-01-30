use crate::models::d2::D2Renderer;
use crate::models::Graph;
use crate::serve::config::{DgConfig, SiteConfig};
use crate::serve::templates::create_environment;
use anyhow::Result;
use minijinja::context;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use rust_embed::Embed;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

// Lazy-initialized D2 renderer (None if d2 binary not available)
static D2_RENDERER: OnceLock<Option<D2Renderer>> = OnceLock::new();

fn get_d2_renderer() -> Option<&'static D2Renderer> {
    D2_RENDERER.get_or_init(|| D2Renderer::new().ok()).as_ref()
}

// Embed static assets for copying to output
#[derive(Embed)]
#[folder = "src/serve/static/"]
#[exclude = "tailwindcss"]
#[exclude = "daisyui*.mjs"]
#[exclude = "input.css"]
struct StaticAssets;

/// Helper to get avatar color class from username (same as server.rs)
fn avatar_color_class(name: &str) -> &'static str {
    let colors = [
        "bg-red-600",
        "bg-orange-600",
        "bg-amber-600",
        "bg-yellow-600",
        "bg-lime-600",
        "bg-green-600",
        "bg-emerald-600",
        "bg-teal-600",
        "bg-cyan-600",
        "bg-sky-600",
        "bg-blue-600",
        "bg-indigo-600",
        "bg-violet-600",
        "bg-purple-600",
        "bg-fuchsia-600",
        "bg-pink-600",
        "bg-rose-600",
    ];
    let hash: usize = name.bytes().map(|b| b as usize).sum();
    colors[hash % colors.len()]
}

/// Helper to convert type code to display name (same as server.rs)
fn type_to_display_name(type_code: &str) -> &'static str {
    match type_code {
        "decision" => "Decision",
        "strategy" => "Strategy",
        "policy" => "Policy",
        "customer" => "Customer",
        "opportunity" => "Opportunity",
        "process" => "Process",
        "hiring" => "Hiring",
        "adr" => "Architecture",
        "incident" => "Incident",
        "runbook" => "Runbook",
        "meeting" => "Meeting",
        "legal" => "Legal",
        _ => "Other",
    }
}

pub fn generate_site(
    graph: &Graph,
    output_dir: &Path,
    docs_dir: &Path,
    base_url: Option<&str>,
) -> Result<()> {
    let base_url = base_url.unwrap_or("");

    fs::create_dir_all(output_dir)?;
    let records_dir = output_dir.join("records");
    fs::create_dir_all(&records_dir)?;
    let users_dir = output_dir.join("users");
    fs::create_dir_all(&users_dir)?;
    let teams_dir = output_dir.join("teams");
    fs::create_dir_all(&teams_dir)?;
    let static_dir = output_dir.join("static");
    fs::create_dir_all(&static_dir)?;
    let api_dir = output_dir.join("api");
    fs::create_dir_all(&api_dir)?;

    // Load site config
    let site_config = SiteConfig::load(docs_dir)?;

    // Load users/teams for mention validation and page generation
    let dg_config = DgConfig::load(docs_dir)?;
    let valid_mentions: HashSet<String> = dg_config
        .users
        .keys()
        .chain(dg_config.teams.keys())
        .cloned()
        .collect();

    let has_users = !dg_config.users.is_empty();

    let env = create_environment();

    // Generate index page
    let index_tmpl = env.get_template("index.html")?;
    let mut records: Vec<_> = graph.all_records().collect();
    records.sort_by(|a, b| b.frontmatter.updated.cmp(&a.frontmatter.updated));
    let records_data: Vec<_> = records.iter().map(|r| record_to_context(r)).collect();

    let mut type_codes: Vec<_> = records_data
        .iter()
        .filter_map(|r| {
            r.get("type")
                .and_then(|t| t.as_str())
                .map(|s| s.to_string())
        })
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    type_codes.sort();

    let record_types: Vec<_> = type_codes
        .iter()
        .map(|code| {
            serde_json::json!({
                "code": code,
                "display": type_to_display_name(code),
            })
        })
        .collect();

    let index_html = index_tmpl.render(context! {
        site => &site_config,
        has_users => has_users,
        current_page => "records",
        records => records_data,
        record_types => record_types,
        base_url => base_url,
    })?;
    fs::write(output_dir.join("index.html"), index_html)?;

    // Generate individual record pages
    let record_tmpl = env.get_template("record.html")?;
    for record in graph.all_records() {
        let mut ctx = record_to_context(record);

        // Add content as HTML using pulldown-cmark
        let content_html =
            markdown_to_html_with_mentions(&record.content, &valid_mentions, base_url);
        ctx.insert(
            "content_html".to_string(),
            serde_json::Value::String(content_html),
        );

        // Add links with titles
        let links: Vec<_> = record
            .frontmatter
            .links
            .all_links()
            .iter()
            .map(|(lt, target)| {
                let title = graph.get(target).map(|r| r.title().to_string());
                serde_json::json!({
                    "type": lt,
                    "target": target,
                    "title": title,
                })
            })
            .collect();
        ctx.insert("links".to_string(), serde_json::Value::Array(links));

        let record_html = record_tmpl.render(context! {
            site => &site_config,
            has_users => has_users,
            current_page => "records",
            record => ctx,
            base_url => base_url,
        })?;
        fs::write(
            records_dir.join(format!("{}.html", record.id())),
            record_html,
        )?;
    }

    // Generate timeline page
    let timeline_tmpl = env.get_template("timeline.html")?;
    let timeline_data = serde_json::json!({
        "nodes": graph.all_records().map(|r| {
            serde_json::json!({
                "id": r.id(),
                "title": r.title(),
                "type": r.record_type().to_string(),
                "created": r.frontmatter.created.to_string(),
                "core": r.frontmatter.core,
            })
        }).collect::<Vec<_>>(),
        "edges": graph.edges.iter().map(|e| {
            serde_json::json!({
                "source": e.from,
                "target": e.to,
                "type": e.link_type,
            })
        }).collect::<Vec<_>>(),
    });
    let timeline_html = timeline_tmpl.render(context! {
        site => &site_config,
        has_users => has_users,
        current_page => "timeline",
        timeline_data => timeline_data.to_string(),
        base_url => base_url,
    })?;
    fs::write(output_dir.join("timeline.html"), timeline_html)?;

    // Generate graph page
    let graph_tmpl = env.get_template("graph.html")?;
    let graph_data = serde_json::json!({
        "nodes": graph.all_records().map(|r| {
            serde_json::json!({
                "id": r.id(),
                "title": r.title(),
                "type": r.record_type().to_string(),
                "status": r.status().to_string(),
                "core": r.frontmatter.core,
            })
        }).collect::<Vec<_>>(),
        "edges": graph.edges.iter().map(|e| {
            serde_json::json!({
                "source": e.from,
                "target": e.to,
                "type": e.link_type,
            })
        }).collect::<Vec<_>>(),
    });
    let graph_html = graph_tmpl.render(context! {
        site => &site_config,
        has_users => has_users,
        current_page => "graph",
        graph_data => graph_data.to_string(),
        base_url => base_url,
    })?;
    fs::write(output_dir.join("graph.html"), graph_html)?;

    // Generate stats page
    let stats_tmpl = env.get_template("stats.html")?;
    let stats = graph.stats();
    let core_count = graph.core_records().len();

    let by_type: Vec<_> = stats
        .by_type
        .iter()
        .map(|(t, c)| {
            serde_json::json!({
                "type": t,
                "type_display": type_to_display_name(t),
                "count": c
            })
        })
        .collect();

    let by_status: Vec<_> = stats
        .by_status
        .iter()
        .map(|(s, c)| serde_json::json!({ "status": s, "count": c }))
        .collect();

    let stats_ctx = serde_json::json!({
        "total_records": stats.total_records,
        "total_edges": stats.total_edges,
        "core": core_count,
        "by_type": by_type,
        "by_status": by_status,
    });
    let stats_html = stats_tmpl.render(context! {
        site => &site_config,
        has_users => has_users,
        current_page => "stats",
        stats => stats_ctx,
        base_url => base_url,
    })?;
    fs::write(output_dir.join("stats.html"), stats_html)?;

    // Generate users list page
    if has_users {
        let users_tmpl = env.get_template("users.html")?;
        let mut users: Vec<_> = dg_config
            .users
            .iter()
            .map(|(username, user)| {
                let avatar_color = avatar_color_class(username);
                serde_json::json!({
                    "username": username,
                    "name": user.display_name(username),
                    "initials": user.initials(username),
                    "avatar_url": user.avatar(username),
                    "avatar_color": avatar_color,
                    "email": user.email,
                    "teams": user.teams,
                    "roles": user.roles,
                    "is_deprecated": user.is_deprecated(),
                    "is_llm": user.roles.contains(&"llm".to_string()),
                })
            })
            .collect();
        users.sort_by(|a, b| {
            let a_name = a["name"].as_str().unwrap_or("");
            let b_name = b["name"].as_str().unwrap_or("");
            a_name.cmp(b_name)
        });

        let users_html = users_tmpl.render(context! {
            site => &site_config,
            has_users => has_users,
            current_page => "users",
            users => users,
            base_url => base_url,
        })?;
        fs::write(output_dir.join("users.html"), users_html)?;

        // Generate individual user pages
        let user_tmpl = env.get_template("user.html")?;
        for (username, user) in &dg_config.users {
            let user_data = serde_json::json!({
                "username": username,
                "name": user.display_name(username),
                "initials": user.initials(username),
                "avatar_url": user.avatar(username),
                "email": user.email,
                "github": user.github,
                "teams": user.teams,
                "roles": user.roles,
                "is_deprecated": user.is_deprecated(),
                "is_llm": user.roles.contains(&"llm".to_string()),
                "deprecated_date": user.deprecated_date,
                "deprecated_note": user.deprecated_note,
            });

            // Find records authored by or mentioning this user
            let mention_pattern = format!("@{}", username);
            let user_display_name = user.display_name(username);

            let mut user_records: Vec<serde_json::Value> = Vec::new();
            let mut seen_ids: HashSet<String> = HashSet::new();

            for record in graph.all_records() {
                let is_author = record.frontmatter.authors.contains(username);
                let roles = record.extract_daci_roles();
                let mut user_daci_role: Option<String> = None;

                for (role, names) in &roles {
                    let is_assigned = names.iter().any(|name| {
                        let name_lower = name.to_lowercase();
                        name_lower.contains(&username.to_lowercase())
                            || name_lower.contains(&user_display_name.to_lowercase())
                            || user_display_name.to_lowercase().contains(&name_lower)
                    });
                    if is_assigned {
                        user_daci_role = Some(role.clone());
                        break;
                    }
                }

                if is_author || user_daci_role.is_some() {
                    if !seen_ids.contains(record.id()) {
                        seen_ids.insert(record.id().to_string());
                        user_records.push(serde_json::json!({
                            "id": record.id(),
                            "title": record.title(),
                            "status": record.status().to_string(),
                            "date": record.frontmatter.created.to_string(),
                            "is_author": is_author,
                            "daci_role": user_daci_role,
                            "core": record.frontmatter.core,
                        }));
                    }
                }
            }

            user_records.sort_by(|a, b| {
                let date_a = a.get("date").and_then(|d| d.as_str()).unwrap_or("");
                let date_b = b.get("date").and_then(|d| d.as_str()).unwrap_or("");
                date_b.cmp(date_a)
            });

            let mentioned_in: Vec<_> = graph
                .all_records()
                .filter(|r| {
                    r.content.contains(&mention_pattern)
                        && !r.frontmatter.authors.contains(username)
                })
                .map(|r| {
                    serde_json::json!({
                        "id": r.id(),
                        "title": r.title(),
                        "status": r.status().to_string(),
                    })
                })
                .collect();

            // Find action items assigned to this user
            let mut action_items: Vec<serde_json::Value> = Vec::new();
            for record in graph.all_records() {
                for (text, completed, owner) in record.extract_action_items() {
                    if let Some(ref owner_name) = owner {
                        if owner_name.to_lowercase() == username.to_lowercase() {
                            action_items.push(serde_json::json!({
                                "record_id": record.id(),
                                "record_title": record.title(),
                                "text": text,
                                "completed": completed,
                            }));
                        }
                    }
                }
            }

            let user_html = user_tmpl.render(context! {
                site => &site_config,
                has_users => has_users,
                current_page => "users",
                user => user_data,
                user_records => user_records,
                mentioned_in => mentioned_in,
                action_items => action_items,
                base_url => base_url,
            })?;
            fs::write(users_dir.join(format!("{}.html", username)), user_html)?;
        }
    }

    // Generate teams list page
    if !dg_config.teams.is_empty() {
        let teams_tmpl = env.get_template("teams.html")?;

        // Build team data
        let core_team_ids = ["founders", "executive", "engineering", "operations"];
        let stakeholder_team_ids = ["board", "investors", "legal", "competitors"];
        let secondary_team_ids: HashSet<String> = dg_config
            .teams
            .iter()
            .filter(|(_, t)| t.lead.is_none())
            .map(|(id, _)| id.clone())
            .collect();

        let build_user_data = |username: &str, user: &crate::models::users::User| {
            let hashtag_teams: Vec<_> = user
                .teams
                .iter()
                .filter(|t| secondary_team_ids.contains(*t))
                .map(|t| {
                    dg_config
                        .teams
                        .get(t)
                        .map(|team| {
                            serde_json::json!({
                                "id": t,
                                "name": team.name,
                            })
                        })
                        .unwrap_or_else(|| serde_json::json!({"id": t, "name": t}))
                })
                .collect();

            let avatar_color = avatar_color_class(username);
            serde_json::json!({
                "username": username,
                "name": user.display_name(username),
                "initials": user.initials(username),
                "avatar_url": user.avatar(username),
                "avatar_color": avatar_color,
                "roles": user.roles,
                "is_deprecated": user.is_deprecated(),
                "deprecated_note": user.deprecated_note,
                "is_current_user": false,
                "hashtag_teams": hashtag_teams,
            })
        };

        let build_team_data = |id: &str, team: &crate::models::teams::Team| {
            let members: Vec<_> = dg_config
                .users
                .iter()
                .filter(|(_, u)| u.teams.contains(&id.to_string()) && !u.is_deprecated())
                .map(|(username, user)| build_user_data(username, user))
                .collect();

            serde_json::json!({
                "id": id,
                "name": team.name,
                "description": team.description,
                "lead": team.lead,
                "parent": team.parent,
                "members": members,
                "member_count": members.len(),
                "is_current_user_team": false,
                "lead_is_current": false,
            })
        };

        let mut core_teams: Vec<_> = dg_config
            .teams
            .iter()
            .filter(|(id, team)| core_team_ids.contains(&id.as_str()) && team.lead.is_some())
            .map(|(id, team)| build_team_data(id, team))
            .collect();
        core_teams.sort_by(|a, b| {
            let a_name = a["name"].as_str().unwrap_or("");
            let b_name = b["name"].as_str().unwrap_or("");
            a_name.cmp(b_name)
        });

        let mut stakeholder_teams: Vec<_> = dg_config
            .teams
            .iter()
            .filter(|(id, _)| stakeholder_team_ids.contains(&id.as_str()))
            .map(|(id, team)| build_team_data(id, team))
            .collect();
        stakeholder_teams.sort_by(|a, b| {
            let a_name = a["name"].as_str().unwrap_or("");
            let b_name = b["name"].as_str().unwrap_or("");
            a_name.cmp(b_name)
        });

        let mut other_teams: Vec<_> = dg_config
            .teams
            .iter()
            .filter(|(id, team)| {
                !core_team_ids.contains(&id.as_str())
                    && !stakeholder_team_ids.contains(&id.as_str())
                    && team.lead.is_some()
            })
            .map(|(id, team)| build_team_data(id, team))
            .collect();
        other_teams.sort_by(|a, b| {
            let a_name = a["name"].as_str().unwrap_or("");
            let b_name = b["name"].as_str().unwrap_or("");
            a_name.cmp(b_name)
        });

        let mut deprecated_users: Vec<_> = dg_config
            .users
            .iter()
            .filter(|(_, u)| u.is_deprecated())
            .map(|(username, user)| build_user_data(username, user))
            .collect();
        deprecated_users.sort_by(|a, b| {
            let a_name = a["name"].as_str().unwrap_or("");
            let b_name = b["name"].as_str().unwrap_or("");
            a_name.cmp(b_name)
        });

        let active_user_count = dg_config
            .users
            .values()
            .filter(|u| !u.is_deprecated())
            .count();

        let teams_html = teams_tmpl.render(context! {
            site => &site_config,
            has_users => has_users,
            current_page => "teams",
            core_teams => core_teams,
            stakeholder_teams => stakeholder_teams,
            other_teams => other_teams,
            deprecated_users => deprecated_users,
            active_user_count => active_user_count,
            current_user_id => Option::<String>::None,
            current_user_team => Option::<String>::None,
            base_url => base_url,
        })?;
        fs::write(output_dir.join("teams.html"), teams_html)?;

        // Generate individual team pages
        let team_tmpl = env.get_template("team.html")?;
        for (id, team) in &dg_config.teams {
            let team_data = serde_json::json!({
                "id": id,
                "name": team.name,
                "lead": team.lead,
                "parent": team.parent,
                "description": team.description,
                "email": team.email,
            });

            let members: Vec<_> = dg_config
                .users
                .iter()
                .filter(|(_, u)| u.teams.contains(id))
                .map(|(username, user)| {
                    serde_json::json!({
                        "username": username,
                        "name": user.display_name(username),
                        "avatar_url": user.avatar(username),
                    })
                })
                .collect();

            // Find records owned by team members
            let team_records: Vec<_> = graph
                .all_records()
                .filter(|r| {
                    r.frontmatter.authors.iter().any(|a| {
                        dg_config
                            .users
                            .get(a)
                            .map(|u| u.teams.contains(id))
                            .unwrap_or(false)
                    })
                })
                .map(|r| {
                    serde_json::json!({
                        "id": r.id(),
                        "title": r.title(),
                        "status": r.status().to_string(),
                        "date": r.frontmatter.created.to_string(),
                    })
                })
                .collect();

            let team_html = team_tmpl.render(context! {
                site => &site_config,
                has_users => has_users,
                current_page => "teams",
                team => team_data,
                members => members,
                team_records => team_records,
                base_url => base_url,
            })?;
            fs::write(teams_dir.join(format!("{}.html", id)), team_html)?;
        }
    }

    // Copy static assets
    for file in StaticAssets::iter() {
        if let Some(content) = StaticAssets::get(&file) {
            let dest = static_dir.join(file.as_ref());
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(dest, content.data.as_ref())?;
        }
    }

    // Generate api/graph.json
    let api_graph = serde_json::json!({
        "nodes": graph.all_records().map(|r| {
            serde_json::json!({
                "id": r.id(),
                "title": r.title(),
                "type": r.record_type().to_string(),
                "status": r.status().to_string(),
                "core": r.frontmatter.core,
                "created": r.frontmatter.created.to_string(),
                "updated": r.frontmatter.updated.to_string(),
            })
        }).collect::<Vec<_>>(),
        "edges": graph.edges.iter().map(|e| {
            serde_json::json!({
                "source": e.from,
                "target": e.to,
                "type": e.link_type,
            })
        }).collect::<Vec<_>>(),
    });
    fs::write(
        api_dir.join("graph.json"),
        serde_json::to_string_pretty(&api_graph)?,
    )?;

    // Copy logo if specified
    if let Some(ref logo_path) = site_config.logo {
        let src = docs_dir.join(logo_path);
        if src.exists() {
            let dest = output_dir.join(logo_path);
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&src, &dest)?;
        }
    }

    // Copy assets directory if it exists
    let assets_src = docs_dir.join("assets");
    if assets_src.exists() {
        let assets_dest = output_dir.join("assets");
        copy_dir_recursive(&assets_src, &assets_dest)?;
    }

    // Apply base_url to all HTML files if specified
    if !base_url.is_empty() {
        apply_base_url_to_html(output_dir, base_url)?;
    }

    Ok(())
}

/// Apply base_url prefix to all href and src attributes in HTML files
fn apply_base_url_to_html(dir: &Path, base_url: &str) -> Result<()> {
    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "html")
                .unwrap_or(false)
        })
    {
        let path = entry.path();
        let content = fs::read_to_string(path)?;

        // Replace href="/ and src="/ with href="{base_url}/ and src="{base_url}/
        let updated = content
            .replace(r#"href="/"#, &format!(r#"href="{}/"#, base_url))
            .replace(r#"href="/static"#, &format!(r#"href="{}/static"#, base_url))
            .replace(
                r#"href="/records"#,
                &format!(r#"href="{}/records"#, base_url),
            )
            .replace(r#"href="/users"#, &format!(r#"href="{}/users"#, base_url))
            .replace(r#"href="/teams"#, &format!(r#"href="{}/teams"#, base_url))
            .replace(
                r#"href="/timeline"#,
                &format!(r#"href="{}/timeline"#, base_url),
            )
            .replace(r#"href="/graph"#, &format!(r#"href="{}/graph"#, base_url))
            .replace(r#"href="/stats"#, &format!(r#"href="{}/stats"#, base_url))
            .replace(r#"href="/api"#, &format!(r#"href="{}/api"#, base_url))
            .replace(r#"href="/assets"#, &format!(r#"href="{}/assets"#, base_url))
            .replace(r#"href="/?"#, &format!(r#"href="{}?"#, base_url))
            .replace(r#"src="/static"#, &format!(r#"src="{}/static"#, base_url))
            .replace(r#"src="/assets"#, &format!(r#"src="{}/assets"#, base_url));

        fs::write(path, updated)?;
    }
    Ok(())
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

fn record_to_context(record: &crate::models::Record) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    map.insert(
        "id".to_string(),
        serde_json::Value::String(record.id().to_string()),
    );
    map.insert(
        "title".to_string(),
        serde_json::Value::String(record.title().to_string()),
    );
    map.insert(
        "type".to_string(),
        serde_json::Value::String(record.record_type().to_string()),
    );
    map.insert(
        "status".to_string(),
        serde_json::Value::String(record.status().to_string()),
    );
    map.insert(
        "created".to_string(),
        serde_json::Value::String(record.frontmatter.created.to_string()),
    );
    map.insert(
        "updated".to_string(),
        serde_json::Value::String(record.frontmatter.updated.to_string()),
    );
    map.insert(
        "core".to_string(),
        serde_json::Value::Bool(record.frontmatter.core),
    );
    map.insert(
        "tags".to_string(),
        serde_json::Value::Array(
            record
                .frontmatter
                .tags
                .iter()
                .map(|t| serde_json::Value::String(t.clone()))
                .collect(),
        ),
    );
    map.insert(
        "authors".to_string(),
        serde_json::Value::Array(
            record
                .frontmatter
                .authors
                .iter()
                .map(|a| serde_json::Value::String(a.clone()))
                .collect(),
        ),
    );
    map
}

/// Convert markdown to HTML using pulldown-cmark (without mention validation)
pub fn markdown_to_html(md: &str) -> String {
    markdown_to_html_with_mentions(md, &HashSet::new(), "")
}

/// Convert markdown to HTML with validated @mentions
pub fn markdown_to_html_with_mentions(
    md: &str,
    valid_mentions: &HashSet<String>,
    base_url: &str,
) -> String {
    // Strip HTML comments before rendering
    let comment_re = Regex::new(r"<!--[\s\S]*?-->").unwrap();
    let cleaned = comment_re.replace_all(md, "");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&cleaned, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Render D2 code blocks to SVG (server-side)
    let html_output = render_d2_blocks(&html_output);

    // Linkify @mentions (only valid ones if validation set provided)
    linkify_mentions(&html_output, valid_mentions, base_url)
}

/// Render D2 code blocks to inline SVG
fn render_d2_blocks(html: &str) -> String {
    // Match <pre><code class="language-d2">...</code></pre> blocks
    let d2_re = Regex::new(r#"<pre><code class="language-d2">([\s\S]*?)</code></pre>"#).unwrap();

    d2_re
        .replace_all(html, |caps: &regex::Captures| {
            let d2_code = &caps[1];
            // Unescape HTML entities in the code
            let unescaped = html_unescape(d2_code);

            match get_d2_renderer() {
                Some(renderer) => match renderer.render_svg(&unescaped) {
                    Ok(svg) => {
                        format!(
                            r#"<div class="d2-container my-4 bg-slate-800 rounded-lg p-4 overflow-x-auto">{}</div>"#,
                            svg
                        )
                    }
                    Err(e) => {
                        // Render error: show code with error message
                        let escaped_code = htmlescape::encode_minimal(&unescaped);
                        let escaped_err = htmlescape::encode_minimal(&e.to_string());
                        format!(
                            r#"<div class="d2-error my-4">
                                <div class="text-red-400 text-sm mb-2">D2 render error: {}</div>
                                <pre class="bg-slate-800 p-4 rounded-lg overflow-x-auto"><code class="language-d2">{}</code></pre>
                            </div>"#,
                            escaped_err, escaped_code
                        )
                    }
                },
                None => {
                    // D2 not available: show code with info message
                    let escaped_code = htmlescape::encode_minimal(&unescaped);
                    format!(
                        r#"<div class="d2-unavailable my-4">
                            <div class="text-slate-500 text-sm mb-2">D2 not installed. <a href="https://d2lang.com" class="text-piper-light hover:underline" target="_blank">Install d2</a> to render this diagram.</div>
                            <pre class="bg-slate-800 p-4 rounded-lg overflow-x-auto"><code class="language-d2">{}</code></pre>
                        </div>"#,
                        escaped_code
                    )
                }
            }
        })
        .to_string()
}

/// Unescape common HTML entities
fn html_unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

/// Convert @username mentions to clickable links (only if user/team exists)
fn linkify_mentions(html: &str, valid_mentions: &HashSet<String>, base_url: &str) -> String {
    let mention_re = Regex::new(r"@([a-zA-Z][a-zA-Z0-9_-]*)").unwrap();
    mention_re
        .replace_all(html, |caps: &regex::Captures| {
            let username = &caps[1];
            // Only create link if valid_mentions is empty (no validation) or username exists
            if valid_mentions.is_empty() || valid_mentions.contains(username) {
                format!(
                    r#"<a href="{}/users/{}" class="mention text-piper-light hover:underline">@{}</a>"#,
                    base_url, username, username
                )
            } else {
                // Keep as plain text for non-existent users
                format!("@{}", username)
            }
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d2_code_block_rendering() {
        let md = r#"# Test

```d2
a -> b
```

After.
"#;
        let html = markdown_to_html(md);

        // Should contain d2-container if D2 is available, or d2-unavailable if not
        assert!(
            html.contains("d2-container") || html.contains("d2-unavailable"),
            "Expected d2 container in output, got: {}",
            html
        );
        // Should NOT contain raw code block with language-d2 class unchanged
        // (it should be wrapped in a d2-container or d2-unavailable div)
        assert!(
            html.contains("d2-") && !html.contains("<pre><code class=\"language-d2\">a -&gt; b"),
            "D2 code should be processed, not left as raw code block"
        );
    }

    #[test]
    fn test_non_d2_code_blocks_preserved() {
        let md = r#"# Test

```rust
fn main() {}
```

After.
"#;
        let html = markdown_to_html(md);

        // Rust code block should be preserved as-is
        assert!(
            html.contains("language-rust"),
            "Rust code block should be preserved"
        );
    }

    #[test]
    fn test_mermaid_code_blocks_preserved() {
        let md = r#"# Test

```mermaid
graph TD
A --> B
```

After.
"#;
        let html = markdown_to_html(md);

        // Mermaid should be preserved for client-side rendering
        assert!(
            html.contains("language-mermaid"),
            "Mermaid code block should be preserved for client-side rendering"
        );
    }

    #[test]
    fn test_html_unescape() {
        assert_eq!(html_unescape("a &lt; b"), "a < b");
        assert_eq!(html_unescape("a &gt; b"), "a > b");
        assert_eq!(html_unescape("a &amp; b"), "a & b");
        assert_eq!(html_unescape("&quot;quoted&quot;"), "\"quoted\"");
    }
}
