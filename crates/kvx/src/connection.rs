use kvx_core::Backend;

pub struct Connection<D>
where
    D: Backend,
{
    driver: D,
}