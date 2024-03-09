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
}

pub trait Command<TCommand> {
    fn execute(command: &TCommand, cqrs: &mut AppState);
}

pub trait Query<TQuery, TResult> {
    fn get(query: &TQuery, cqrs: &AppState) -> TResult;
}
