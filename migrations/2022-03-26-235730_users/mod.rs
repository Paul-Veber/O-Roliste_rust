/// Handle up migrations 
fn up(migr: &mut Migration) {
    migr.create_table("users", |t| {
        t.add_column("id", types::serial().primary(true).increments(true));
        t.add_column("pseudo", types::varchar(255));
        t.add_column("email", types::varchar(255).unique(true));
        t.add_column("password", types::varchar(255));
        t.add_column("age", types::integer().nullable(true));
        t.add_column("created_at", types::datetime());
        t.add_column("updated_at", types::datetime().nullable(true));
        
    });

} 

/// Handle down migrations 
fn down(migr: &mut Migration) {
    migr.drop_table("users");
} 
