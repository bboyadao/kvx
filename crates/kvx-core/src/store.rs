use crate::{
    Driver,
    Executor,
};


pub struct Store<D>
where
    D: Driver
{

    executor: Executor<D>,

}



impl<D> Store<D>
where
    D: Driver
{

    pub fn new(
        driver:D
    )->Self {

        Self {
            executor:
                Executor::new(driver)
        }

    }



    pub fn executor(
        &self
    )->&Executor<D>{

        &self.executor

    }

}