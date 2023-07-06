use bevy::asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset};
use crate::world::components::CellType::{ELECTRON, EMPTY, WIRE};
use crate::world::resources::World;

pub struct WorldLoader;

impl AssetLoader for WorldLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        Box::pin(async move {
            if let Ok(content) = String::from_utf8(Vec::from(bytes)) {
                let world = World::from_string(content)?;

                let loaded_asset = LoadedAsset::new(world);
                load_context.set_default_asset(loaded_asset);
                Ok(())
            } else {
                Err(Error::msg("Failed read level file"))
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["level"];
        EXTENSIONS
    }
}