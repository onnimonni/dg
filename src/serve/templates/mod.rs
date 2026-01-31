use minijinja::Environment;

mod base;
mod edit;
mod graph;
mod index;
mod record;
mod stats;
mod teams;
mod timeline;
mod users;

pub use base::BASE_TEMPLATE;
pub use edit::EDIT_TEMPLATE;
pub use graph::GRAPH_TEMPLATE;
pub use index::INDEX_TEMPLATE;
pub use record::RECORD_TEMPLATE;
pub use stats::STATS_TEMPLATE;
pub use teams::{TEAMS_TEMPLATE, TEAM_HISTORY_TEMPLATE, TEAM_TEMPLATE};
pub use timeline::TIMELINE_TEMPLATE;
pub use users::{USERS_TEMPLATE, USER_TEMPLATE};

pub fn create_environment() -> Environment<'static> {
    let mut env = Environment::new();
    env.add_template("base.html", BASE_TEMPLATE).unwrap();
    env.add_template("index.html", INDEX_TEMPLATE).unwrap();
    env.add_template("record.html", RECORD_TEMPLATE).unwrap();
    env.add_template("graph.html", GRAPH_TEMPLATE).unwrap();
    env.add_template("stats.html", STATS_TEMPLATE).unwrap();
    env.add_template("timeline.html", TIMELINE_TEMPLATE)
        .unwrap();
    env.add_template("edit.html", EDIT_TEMPLATE).unwrap();
    env.add_template("users.html", USERS_TEMPLATE).unwrap();
    env.add_template("user.html", USER_TEMPLATE).unwrap();
    env.add_template("teams.html", TEAMS_TEMPLATE).unwrap();
    env.add_template("team.html", TEAM_TEMPLATE).unwrap();
    env.add_template("team_history.html", TEAM_HISTORY_TEMPLATE)
        .unwrap();
    env
}
