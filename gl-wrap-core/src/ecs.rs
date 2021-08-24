use std::any::TypeId;
use std::any::Any;
use std::collections::HashMap;
use crate::component::Component;
use crate::types::Id;
//use crate::instance::GameInstance;

pub struct ECS {
    /// Lists of all Components that exist in the system.
    components: HashMap<TypeId, Box<dyn Any>>,
    /// Hashmap of Ids by Entity name
    entity_ids: HashMap<String, Id>,
    /// List of Entities. Index is their Id.
    entities: Vec<Entity>,
    /// List of all updaters of each individual Component.
    updaters: Vec<fn(&mut HashMap<TypeId, Box<dyn Any>>)>,
    //instance: Box<GameInstance>
}

impl ECS {
    /// Creates a new ECS system.
    pub fn new() -> Self {
        ECS {
            components: HashMap::new(),
            entity_ids: HashMap::new(),
            entities: Vec::new(),
            updaters: Vec::new(),
            //instance: Box::new(GameInstance::new())
        }
    }

    /// Helper function for adding a Component.
    ///
    /// When adding a component, it first checks to see if any Component Vec is available,
    /// and if not, then inserts a new Vec of the associated Vec.
    /// Then, it also inserts an updater function for the associated Component,
    /// which is will be run on update.
    fn add_component<C: Component + 'static>(&mut self, entity: &mut Entity, data_type: TypeId) -> (Id, &mut C) {
        if !self.components.contains_key(&data_type) {
            self.components.insert(data_type, Box::new(Vec::new() as Vec<C>));
            self.updaters.push(ECS::updater::<C>);
        }

        let comps = self.components.get_mut(&data_type)
            .unwrap()
            .as_mut()
            .downcast_mut::<Vec<C>>()
            .unwrap();

        comps.push(C::create(entity));
        let index = comps.len() - 1;
        let comp = comps.get_mut(index).unwrap();

        (index, comp)
    }

    /// Helper function for getting a Component.
    fn get_component<C: Component + 'static>(&mut self, comp_id: TypeId, index: Id) -> Option<&mut C> {
        let comps = self.components
            .get_mut(&comp_id)
            .unwrap()
            .as_mut()
            .downcast_mut::<Vec<C>>()
            .unwrap();

        comps.get_mut(index)
    }

    /// Adds a new Entity to the system.
    pub fn new_entity(&mut self, name: &str) -> &mut Entity {
        let id = self.entities.len();
        let entity = Entity::new(name, self as *mut _);
        self.entity_ids.insert(name.to_string(), id);
        self.entities.push(entity);
        self.entities.get_mut(id).unwrap()
    }

    /// Updates the ECS.
    pub fn update(&mut self) {
        for updater in &mut self.updaters {
            updater(&mut self.components);
        }
    }
    
    /// Used for updating the components, pushed in AddComponent().
    fn updater<C: Component + 'static>(map: &mut HashMap<TypeId, Box<dyn Any>>) {
        let data_type = TypeId::of::<C>();
        let comps = map.get_mut(&data_type)
            .unwrap()
            .as_mut()
            .downcast_mut::<Vec<C>>()
            .unwrap();

        for comp in comps {
            comp.update();
        }
    }
}

pub struct Entity {
    /// Name of Entity
    name: String,
    /// Hashmap of Ids defined by TypeId
    comp_index: HashMap<TypeId, Id>,
    /// Pointer to ECS system
    ecs: *mut ECS,
}

impl Entity {
    /// Helper function to create a new Entity.
    fn new(name: &str, ecs: *mut ECS) -> Self {
        Entity {
            name: name.to_string(),
            comp_index: HashMap::new(),
            ecs: ecs,
        }
    }

    /// Add a Component to the Entity.
    pub fn add_component<C: Component + 'static>(&mut self, f: &dyn Fn(&mut C)) {
        let data_type = TypeId::of::<C>(); 
        let (index, comp) = unsafe {
            self.ecs.as_mut().unwrap().add_component::<C>(self, data_type)
        };

        self.comp_index.insert(data_type, index);

        f(comp);
    }

    /// Get a Component from the Entity.
    pub fn get_component<C: Component + 'static>(&self) -> Option<&mut C> {
        let data_type = TypeId::of::<C>(); 
        let index = self.comp_index.get(&data_type);
        if index.is_none() {
            return None;
        }

        unsafe {
            self.ecs
                .as_mut()
                .unwrap()
                .get_component::<C>(data_type, *index.unwrap())
        }
    }

    /// Delete the Entity.
    pub fn delete(self) {
        unsafe {
            let ecs = self.ecs
                .as_mut()
                .unwrap();
            let id = ecs
                .entity_ids[&self.name];

            ecs.entities.remove(id);
        }
    }

    /*pub fn scene(&mut self) -> &mut ECS {
        unsafe { &mut *self.ecs as &mut _ }
    }*/
}
