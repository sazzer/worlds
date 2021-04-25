use argonautica::Hasher;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::SeedData;

#[derive(Debug)]
pub struct SeedUser {
    pub user_id:      Uuid,
    pub version:      Uuid,
    pub created:      DateTime<Utc>,
    pub updated:      DateTime<Utc>,
    pub username:     String,
    pub display_name: String,
    pub email:        String,
    pub password:     String,
}

impl Default for SeedUser {
    fn default() -> Self {
        let now = Utc::now();

        Self {
            user_id:      Uuid::new_v4(),
            version:      Uuid::new_v4(),
            created:      now,
            updated:      now,
            username:     Uuid::new_v4().to_string(),
            display_name: "Test User".to_owned(),
            email:        format!("{}@example.com", Uuid::new_v4()),
            password:     "".to_owned(),
        }
    }
}

impl SeedUser {
    pub fn with_password(self, password: &str) -> Self {
        let hash = Hasher::default()
            .with_password(password)
            .opt_out_of_secret_key(true)
            .hash()
            .unwrap();

        Self { password: hash, ..self }
    }
}

impl SeedData for SeedUser {
    fn sql(&self) -> &str {
        "INSERT INTO users(user_id, version, created, updated, username, display_name, email, password) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    }

    fn binds(&self) -> Vec<&(dyn postgres_types::ToSql + Sync)> {
        vec![
            &self.user_id,
            &self.version,
            &self.created,
            &self.updated,
            &self.username,
            &self.display_name,
            &self.email,
            &self.password,
        ]
    }
}
