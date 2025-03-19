use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use secrecy::SecretBox;

use crate::commands::auth;
use crate::commands::db;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, env = "SUPABASE_PROJECT_REF")]
    pub project_ref: Option<String>,
    #[arg(short, long, env = "SUPABASE_ANON_KEY")]
    pub anon_key: Option<String>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },
    Setup {
        #[command(flatten)]
        sensitive_args: SetupSensitiveArgs,
        #[command(subcommand)]
        command: SetupCommands,
    },
}

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Sign up without a password
    #[command(name = "sign-up-no-password")]
    SignUpNoPassword,
    /// Sign in as a non-existent user
    #[command(name = "sign-in-fake-user")]
    SignInFakeUser,
    /// Call an admin endpoint with an anon key
    #[command(name = "call-admin-with-anon")]
    CallAdminWithAnon,
}

#[derive(Subcommand)]
pub enum DbCommands {
    /// Select from a non-existent table (404 error)
    #[command(name = "select-nonexistent-table")]
    SelectNonexistentTable,
    /// Make API call with incomplete anon key (401 error)
    #[command(name = "incomplete-anon-key")]
    IncompleteAnonKey,
}

#[derive(Args)]
pub struct SetupSensitiveArgs {
    /// Database connection string (sensitive)
    #[arg(long, env = "PGMETA_CONNECTION_STRING", hide_env_values = true)]
    pub encrypted_connection_string: Option<String>,
}

#[derive(Subcommand)]
pub enum SetupCommands {
    /// Create a timeout function to use for some DB tests
    #[command(name = "create-timeout-function")]
    CreateTimeoutFunction,
}

impl Cli {
    pub async fn execute(&self) -> Result<()> {
        let project_ref = self.project_ref.clone().expect("Missing Supabase project reference. Set SUPABASE_PROJECT_REF env var or use --project-ref");
        let anon_key = self
            .anon_key
            .clone()
            .expect("Missing Supabase anon key. Set SUPABASE_ANON_KEY env var or use --anon-key");

        match &self.command {
            Commands::Auth { command } => match command {
                AuthCommands::SignUpNoPassword => {
                    auth::sign_up_no_password(&project_ref, &anon_key).await
                }
                AuthCommands::SignInFakeUser => {
                    auth::sign_in_fake_user(&project_ref, &anon_key).await
                }
                AuthCommands::CallAdminWithAnon => {
                    auth::call_admin_with_anon(&project_ref, &anon_key).await
                }
            },
            Commands::Db { command } => match command {
                DbCommands::SelectNonexistentTable => {
                    db::select_nonexistent_table(&project_ref, &anon_key).await
                }
                DbCommands::IncompleteAnonKey => {
                    db::incomplete_anon_key(&project_ref, &anon_key).await
                }
            },
            Commands::Setup {
                sensitive_args,
                command,
            } => {
                let db_connection = sensitive_args.encrypted_connection_string.clone().expect("Missing encrypted connection string. Set PGMETA_CONNECTION_STRING env var or use --encrypted-connection-string");
                let db_connection = SecretBox::new(Box::new(db_connection));

                match command {
                    SetupCommands::CreateTimeoutFunction => {
                        db::setup::create_timeout_function(&project_ref, &anon_key, &db_connection)
                            .await
                    }
                }
            }
        }
    }
}
