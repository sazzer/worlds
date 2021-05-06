use postgres_types::ToSql;

/// Trait that can be implemented by anything able to contribute seed data to the database
pub trait SeedData: std::fmt::Debug {
    /// Generate the SQL that will be used to insert the data into the database
    fn sql(&self) -> &str;

    /// Generate the binds that are used with the SQL.
    /// If not implemented then the default returns the empty set.
    fn binds(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![]
    }
}
