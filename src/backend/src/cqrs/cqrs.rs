use crate::backend::Backend;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CQRS {
    app_state: Rc<RefCell<Backend>>,
}

impl CQRS {
    pub fn new(app_state: Rc<RefCell<Backend>>) -> Self {
        Self { app_state }
    }

    pub fn execute<TCommand: Command<TCommand>>(&mut self, command: &TCommand) {
        TCommand::execute(command, self.app_state.clone());
    }

    pub fn get<TQuery: Query<TQuery, TResult>, TResult>(&self, query: &TQuery) -> TResult {
        TQuery::get(query, self.app_state.clone())
    }

    pub fn handle<TOperation: Operation<TOperation, TResult>, TResult>(
        &mut self,
        operation: &TOperation,
    ) -> TResult {
        TOperation::handle(operation, self.app_state.clone())
    }
}

pub trait Command<TCommand> {
    fn execute(command: &TCommand, app_state: Rc<RefCell<Backend>>);
}

pub trait Query<TQuery, TResult> {
    fn get(query: &TQuery, app_state: Rc<RefCell<Backend>>) -> TResult;
}

pub trait Operation<TOperation, TResult> {
    fn handle(query: &TOperation, app_state: Rc<RefCell<Backend>>) -> TResult;
}
