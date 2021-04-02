use std::collections::HashMap;
use testcontainers::core::Port;
use testcontainers::{Container, Docker, Image, WaitForMessage};

#[derive(Debug)]
pub struct Postgres {
    arguments: PostgresArgs,
    env_vars: HashMap<String, String>,
    ports: Option<Vec<Port>>,
}

#[derive(Default, Debug, Clone)]
pub struct PostgresArgs {}

impl IntoIterator for PostgresArgs {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        vec![].into_iter()
    }
}

impl Default for Postgres {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("POSTGRES_DB".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_HOST_AUTH_METHOD".into(), "trust".into());

        Self {
            arguments: PostgresArgs::default(),
            env_vars,
            ports: None,
        }
    }
}
impl Image for Postgres {
    type Args = PostgresArgs;
    type EnvVars = HashMap<String, String>;
    type Volumes = HashMap<String, String>;
    type EntryPoint = std::convert::Infallible;

    fn descriptor(&self) -> String {
        "postgres:12.4-alpine".to_owned()
    }

    fn wait_until_ready<D: Docker>(&self, container: &Container<'_, D, Self>) {
        container
            .logs()
            .stderr
            .wait_for_message("database system is ready to accept connections")
            .unwrap();
    }

    fn args(&self) -> Self::Args {
        self.arguments.clone()
    }

    fn volumes(&self) -> Self::Volumes {
        HashMap::new()
    }

    fn env_vars(&self) -> Self::EnvVars {
        self.env_vars.clone()
    }

    fn with_args(self, arguments: Self::Args) -> Self {
        Self { arguments, ..self }
    }
}
