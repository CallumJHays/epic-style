use std::collections::HashMap as Map;

use crate::component::{MountID, IsoWrapper};


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub components: Map<MountID, (
        IsoWrapper,
        Option<IsoWrapper>
    )>
}


impl Config {
    pub fn new() -> Self {
        Config {
            components: Map::new()
        }
    }
}

// see stdweb::js_serializable!;
#[cfg(not(target_os="linux"))]
js_serializable!(Config);

#[cfg(not(target_os="linux"))]
js_deserializable!(Config);