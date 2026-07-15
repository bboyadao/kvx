use kvx_core::Driver;

pub struct Connection<D>
where
    D: Driver,
{
    driver: D,
}

impl<D> Connection<D>
where
    D: Driver,
{
    pub fn new(driver: D) -> Self {
        Self { driver }
    }

    pub fn driver(self) -> D {
        self.driver
    }
}