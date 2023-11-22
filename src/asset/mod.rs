use crate::{
    ecs::{Resource, ResourceManager},
    shared::{ext::path::PathExt, ResourceId, ResourceType},
};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    path::{Path, PathBuf},
};

pub trait Asset: 'static {}

pub type AssetId = ResourceId;
pub type AssetType = ResourceType;

pub struct AssetInfo {
    pub id: AssetId,
    pub name: String,
    pub path: PathBuf,
}

pub struct ImportContext<'a> {
    info: AssetInfo,
    resources: &'a ResourceManager,
    assets: &'a mut AssetDatabase,
}

impl<'a> ImportContext<'a> {
    pub fn new(
        info: AssetInfo,
        resources: &'a ResourceManager,
        assets: &'a mut AssetDatabase,
    ) -> Self {
        Self {
            info,
            resources,
            assets,
        }
    }

    pub fn info(&self) -> &AssetInfo {
        &self.info
    }

    pub fn resources(&self) -> &ResourceManager {
        self.resources
    }

    pub fn assets(&mut self) -> &mut AssetDatabase {
        self.assets
    }
}

pub struct AssetStorage<T: Asset> {
    assets: HashMap<AssetId, T>,
}

impl<T: Asset> AssetStorage<T> {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn get(&self, id: &AssetId) -> Option<&T> {
        self.assets.get(id)
    }

    pub fn insert(&mut self, id: AssetId, asset: T) {
        self.assets.insert(id, asset);
    }

    pub fn remove(&mut self, id: &AssetId) -> Option<T> {
        self.assets.remove(id)
    }

    pub fn clear(&mut self) {
        self.assets.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&AssetId, &T)> {
        self.assets.iter()
    }
}

pub struct AssetDatabase {
    storages: HashMap<AssetType, Box<dyn Any>>,
}

impl AssetDatabase {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }

    pub fn get<T: Asset>(&self, id: &AssetId) -> Option<&T> {
        let asset_type = TypeId::of::<T>().into();
        self.storages
            .get(&asset_type)
            .and_then(|storage| storage.downcast_ref::<AssetStorage<T>>())
            .and_then(|storage| storage.get(id))
    }

    pub fn insert<T: Asset>(&mut self, id: AssetId, asset: T) {
        let asset_type = TypeId::of::<T>().into();
        let storage = self
            .storages
            .entry(asset_type)
            .or_insert_with(|| Box::new(AssetStorage::<T>::new()));
        storage
            .downcast_mut::<AssetStorage<T>>()
            .unwrap()
            .insert(id, asset);
    }

    pub fn remove<T: Asset>(&mut self, id: &AssetId) -> Option<T> {
        let asset_type = TypeId::of::<T>().into();
        self.storages
            .get_mut(&asset_type)
            .and_then(|storage| storage.downcast_mut::<AssetStorage<T>>())
            .and_then(|storage| storage.remove(id))
    }

    pub fn iter<T: Asset>(&self) -> Option<impl Iterator<Item = (&AssetId, &T)>> {
        let asset_type = TypeId::of::<T>().into();
        self.storages
            .get(&asset_type)
            .and_then(|storage| storage.downcast_ref::<AssetStorage<T>>())
            .map(|storage| storage.iter())
    }

    pub fn clear(&mut self) {
        self.storages.clear();
    }
}

impl AssetDatabase {
    pub fn load(base_path: &Path, resources: &ResourceManager, importers: &ImporterRepo) {
        let mut db = resources.resource_mut::<AssetDatabase>();
        let sorted_importers: HashMap<&str, Vec<&Box<dyn BaseImporter>>> = importers.sort();

        AssetDatabase::load_inner(base_path, &mut db, resources, &sorted_importers);
    }

    fn load_inner(
        base_path: &Path,
        db: &mut AssetDatabase,
        resources: &ResourceManager,
        importers: &HashMap<&str, Vec<&Box<dyn BaseImporter>>>,
    ) {
        let read_dir = std::fs::read_dir(base_path).expect("Failed to read path: {path}");

        for entry in read_dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let _ = AssetDatabase::load_inner(&path, db, resources, importers);
                } else if path.is_file() {
                    let ext = path.extension_str();
                    if let Some(importers) = importers.get(ext) {
                        for importer in importers {
                            let id = path
                                .trim_prefix(base_path.with_extension("").to_str().unwrap())
                                .into();
                            let info = AssetInfo {
                                id,
                                name: path.file_stem().unwrap().to_str().unwrap().to_string(),
                                path: path.clone(),
                            };
                            let mut ctx = ImportContext::new(info, resources, db);
                            importer.import(&mut ctx);
                        }
                    }
                }
            }
        }
    }
}

impl Resource for AssetDatabase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub trait BaseImporter: 'static {
    fn import(&self, context: &mut ImportContext);
    fn extensions(&self) -> &'static [&'static str] {
        &[]
    }
}

pub trait AssetImporter<T: Asset>: BaseImporter {}

pub struct ImporterRepo {
    importers: HashMap<AssetType, Box<dyn BaseImporter>>,
}

impl ImporterRepo {
    pub fn new() -> Self {
        Self {
            importers: HashMap::new(),
        }
    }

    pub fn add_importer<T: Asset, U: AssetImporter<T>>(&mut self, importer: U) {
        let asset_type = TypeId::of::<T>().into();
        self.importers.insert(asset_type, Box::new(importer));
    }

    pub fn sort(&self) -> HashMap<&str, Vec<&Box<dyn BaseImporter>>> {
        let mut sorted_importers: HashMap<&str, Vec<&Box<dyn BaseImporter>>> = HashMap::new();
        for (_, importer) in &self.importers {
            for extension in importer.extensions() {
                sorted_importers
                    .entry(extension)
                    .or_insert_with(Vec::new)
                    .push(importer);
            }
        }

        sorted_importers
    }
}
