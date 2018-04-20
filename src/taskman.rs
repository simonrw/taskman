use opts::Opts;
use db::establish_connection;
use priority::Priority;
use errors::Result;
use diesel::sqlite::SqliteConnection;
use db::models::NewTask;

pub struct TaskMan {
    connection: SqliteConnection,
    opts: Opts,
}

impl TaskMan {
    pub fn from_opts(opts: Opts) -> Result<Self> {
        let connection = establish_connection()?;
        Ok(TaskMan { connection, opts })
    }

    pub fn run(&mut self) -> Result<()> {
        // TODO: remove this clone
        let opts = self.opts.clone();
        match opts {
            Opts::Add {
                description,
                priority,
                ..
            } => {
                self.add_task(description, priority)?;
            }
        }

        Ok(())
    }

    fn add_task(&mut self, description: String, priority: Option<Priority>) -> Result<()> {
        NewTask::new(&description, priority).create(&self.connection)?;
        Ok(())
    }
}
