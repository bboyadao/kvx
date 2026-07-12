use kvx_types::{
    Key,
    Value,
};


#[derive(Debug)]
pub enum Command {

    Put {
        key: Key,
        value: Value,
    },

    Get {
        key: Key,
    },

    Delete {
        key: Key,
    },

}


impl Command {


    pub fn put(
        key: impl Into<Key>,
        value: impl Into<Value>,
    ) -> Self {

        Self::Put {
            key: key.into(),
            value: value.into(),
        }

    }



    pub fn get(
        key: impl Into<Key>
    ) -> Self {

        Self::Get {
            key: key.into(),
        }

    }



    pub fn delete(
        key: impl Into<Key>
    ) -> Self {

        Self::Delete {
            key: key.into(),
        }

    }

}