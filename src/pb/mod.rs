// @generated
// @@protoc_insertion_point(attribute:gmx)
pub mod gmx {
    include!("gmx.rs");
    // @@protoc_insertion_point(gmx)
}
pub mod sf {
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod sink {
            pub mod entity {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.entity.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.entity.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.entity.v1)
                }
            }
            pub mod service {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.service.v1)
                // pub mod v1 {
                //     include!("sf.substreams.sink.service.v1.rs");
                //     // @@protoc_insertion_point(sf.substreams.sink.service.v1)
                // }
            }
            pub mod sql {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.sql.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.sql.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.sql.v1)
                }
            }
        }
    }
}
