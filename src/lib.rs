pub mod auth {
    tonic::include_proto!("auth");
}

pub mod searcher {
    tonic::include_proto!("searcher");
}

pub mod bundle {
    tonic::include_proto!("bundle");
}

pub mod packet {
    tonic::include_proto!("packet");
}

pub mod shared {
    tonic::include_proto!("shared");
}
