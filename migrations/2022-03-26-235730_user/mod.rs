/// Handle up migrations 
fn up(migr: &mut Migration) {
    migr.create_table("user", |t| {
        t.add_column("id", types::text().primary(true));
        t.add_column("name", types::varchar(255));
        t.add_column("email", types::varchar(255));
        t.add_column("age", types::integer().nullable(true));
    });

} 

/// Handle down migrations 
fn down(migr: &mut Migration) {
    migr.drop_table("user");
} 
