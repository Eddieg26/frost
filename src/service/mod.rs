use crate::shared::ResourceType;
use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Clone)]
pub struct ServiceRepo {
    repo: HashMap<ResourceType, Rc<RefCell<Box<dyn Service>>>>,
}

impl ServiceRepo {
    pub fn new() -> Self {
        Self {
            repo: HashMap::new(),
        }
    }

    pub fn register<T: Service>(&mut self, service: T) {
        self.repo.insert(
            TypeId::of::<T>().into(),
            Rc::new(RefCell::new(Box::new(service))),
        );
    }

    pub fn get<T: Service>(&self) -> &Rc<RefCell<Box<dyn Service>>> {
        self.repo.get(&TypeId::of::<T>().into()).unwrap()
    }

    pub fn service<T: Service>(&self) -> Ref<T> {
        let service = self.repo.get(&TypeId::of::<T>().into()).unwrap().borrow();

        Ref::map(service, |s| s.as_any().downcast_ref::<T>().unwrap())
    }

    pub fn service_mut<T: Service>(&self) -> RefMut<T> {
        let service = self
            .repo
            .get(&TypeId::of::<T>().into())
            .unwrap()
            .borrow_mut();

        RefMut::map(service, |s| s.as_any_mut().downcast_mut::<T>().unwrap())
    }
}

pub trait Service: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ServiceRef<'a, T: Service> {
    service: Ref<'a, T>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: Service> ServiceRef<'a, T> {
    pub fn new(services: &'a ServiceRepo) -> Self {
        Self {
            service: services.service::<T>(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T> Deref for ServiceRef<'a, T>
where
    T: Service,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

pub struct ServiceRefMut<'a, T: Service> {
    service: RefMut<'a, T>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: Service> ServiceRefMut<'a, T> {
    pub fn new(services: &'a ServiceRepo) -> Self {
        Self {
            service: services.service_mut::<T>(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T> Deref for ServiceRefMut<'a, T>
where
    T: Service,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl<'a, T> DerefMut for ServiceRefMut<'a, T>
where
    T: Service,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.service
    }
}
