#[derive(Debug, Clone)]
pub struct RedisOptions {

    pub url:String,

}


impl RedisOptions {

    pub fn new(
        url: impl Into<String>
    ) -> Self {

        Self {
            url:url.into()
        }

    }

}