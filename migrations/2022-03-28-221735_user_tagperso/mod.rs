/// Handle up migrations
fn up(migr: &mut Migration) {
    migr.create_table("user_tagperso", |t| {
        t.add_column(
            "user_id",
            types::foreign(
                "users",
                "id",
                types::ReferentialAction::Unset,
                types::ReferentialAction::Cascade,
            ).primary(true),
        );
        t.add_column(
            "tagperso_id",
            types::foreign(
                "tagperso",
                "id",
                types::ReferentialAction::Unset,
                types::ReferentialAction::Cascade,
            ).primary(true),
        );
    });
}

/// Handle down migrations
fn down(migr: &mut Migration) {
    migr.drop_table("user_tagperso")
}
