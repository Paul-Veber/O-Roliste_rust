/// Handle up migrations
fn up(migr: &mut Migration) {
    migr.create_table("tagsperso", |t| {
        t.add_column("id", types::serial().primary(true).increments(true));
        t.add_column("name", types::varchar(255));
        t.add_column("type", types::varchar(255).nullable(true));
    });
}

/// Handle down migrations
fn down(migr: &mut Migration) {
    migr.drop_table("tagsperso");
}
