use crate::app_state::AppState;

pub struct CQRS<'a> {
    app_state: &'a mut AppState,
}

impl<'a> CQRS<'a> {
    pub fn new(app_state: &'a mut AppState) -> Self {
        Self { app_state }
    }
    
    pub fn execute<TCommand: Command<TCommand>>(&mut self, command: &TCommand) {
        TCommand::execute(command, &mut self.app_state);
    }
    
    pub fn get<TQuery: Query<TQuery, TResult>, TResult>(&self, query: &TQuery) -> TResult {
        TQuery::get(query, self.app_state)
    }
    
    pub fn handle<TOperation: Operation<TOperation, TResult>, TResult>(&mut self, operation: &TOperation) -> TResult {
        TOperation::handle(operation, &mut self.app_state)
    }
}

pub trait Command<TCommand> {
    fn execute(command: &TCommand, app_state: &mut AppState);
}

pub trait Query<TQuery, TResult> {
    fn get(query: &TQuery, app_state: &AppState) -> TResult;
}

pub trait Operation<TOperation, TResult> {
    fn handle(query: &TOperation, app_state: &mut AppState) -> TResult;
}
