use crate::{
    Backend,
    Executor,
};


pub struct Store<B>
where
    B: Backend,
{
    executor: Executor<B>,
}


impl<B> Store<B>
where
    B: Backend,
{

    pub fn new(
        backend: B,
    ) -> Self {

        Self {
            executor:
                Executor::new(backend)
        }

    }


    pub fn executor(
        &self,
    ) -> &Executor<B> {

        &self.executor

    }

}