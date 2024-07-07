#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate serde;
extern crate serde_json;

pub mod types {

    pub mod catalogs {
        pub mod v1 {
            include!("gen/unitycatalog.catalogs.v1.rs");
        }
    }

    pub mod schemas {
        pub mod v1 {
            include!("gen/unitycatalog.schemas.v1.rs");
        }
    }

    pub mod tables {
        pub mod v1 {
            include!("gen/unitycatalog.tables.v1.rs");
        }
    }

    pub mod volumes {
        pub mod v1 {
            include!("gen/unitycatalog.volumes.v1.rs");
        }
    }

    pub mod functions {
        pub mod v1 {
            include!("gen/unitycatalog.functions.v1.rs");
        }
    }
}

pub mod models;
