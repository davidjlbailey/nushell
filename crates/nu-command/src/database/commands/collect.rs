use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, Type,
};

use super::super::SQLiteDatabase;

#[derive(Clone)]
pub struct CollectDb;

impl Command for CollectDb {
    fn name(&self) -> &str {
        "collect"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Custom("database".into()))
    }

    fn usage(&self) -> &str {
        "Collects a query from a database database connection"
    }

    fn input_type(&self) -> Type {
        Type::Custom("database".into())
    }

    fn output_type(&self) -> Type {
        Type::Any
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Collect from a select query",
            example: "open foo.db | into db | select a | from table_1 | collect",
            result: None,
        }]
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["database", "collect"]
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let db = SQLiteDatabase::try_from_pipeline(input, call.head)?;

        db.collect(call.head)
            .map(IntoPipelineData::into_pipeline_data)
    }
}
