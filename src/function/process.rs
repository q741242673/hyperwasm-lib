use std::{marker::PhantomData, time::Duration};
use serde::{Deserialize, Serialize};
use crate::{serializer::{Serializer, Bincode}, tag::Tag, ProcessConfig};
use crate::host::{self,  process_id};

pub trait IntoProcess<M, S> {
    type Process;

    fn spawn<C>(
        capture: C,
        entry: fn(C, Self),
        link: Option<Tag>,
        config: Option<&ProcessConfig>,
        //node: Option<u64>,
    ) -> Self::Process
    where
        S: Serializer<C> ;
}

pub trait NoLink {}

#[derive(Serialize, Deserialize)]
pub struct Process<M, S = Bincode> {
    // node_id: u64,
    id: u64,
    #[serde(skip_serializing, default)]
    serializer_type: PhantomData<(M, S)>,
}

impl<M, S> Process<M, S> {
    pub(crate) fn new(process_id: u64) -> Self {
        Self {
            // node_id,
            id: process_id,
            serializer_type: PhantomData,
        }
    }

    pub fn this() -> Self {
        Self::new(process_id())
    }
    /// Spawn a process.
    pub fn spawn<C, T>(capture: C, entry: fn(C, T)) -> T::Process
    where
        S: Serializer<C> ,
        T: IntoProcess<M, S>,
        T: NoLink,
    {
        T::spawn(capture, entry, None, None)
    }


    /// Spawn a process with a custom configuration.
    pub fn spawn_config<C, T>(config: &ProcessConfig, capture: C, entry: fn(C, T)) -> T::Process
    where
        S: Serializer<C> ,
        T: IntoProcess<M, S>,
        T: NoLink,
    {
        T::spawn(capture, entry, None, Some(config))
    }


    


    pub fn spawn_link<C, T>(capture: C, entry: fn(C, T)) -> T::Process
    where
        S: Serializer<C> ,
        T: IntoProcess<M, S>,
    {
        T::spawn(capture, entry, Some(Tag::new()), None)
    }





    pub fn id(&self) -> u64 {
        self.id
    }




   
    pub fn link(&self) {
        unsafe { host::api::process::link(0, self.id) };
    }

    
    pub fn unlink(&self) {
        unsafe { host::api::process::unlink(self.id) };
    }

    
    pub fn kill(&self) {
        unsafe { host::api::process::kill(self.id) };
    }

    
    pub fn register(&self, name: &str) {
        // Encode type information in name
        let name = format!(
            "{} + Process + {}/{}",
            name,
            std::any::type_name::<M>(),
            std::any::type_name::<S>()
        );
        unsafe { host::api::registry::put(name.as_ptr(), name.len(), self.id) };
    }


}


impl<M, S> Process<M, S>
where
    S: Serializer<M>,
{

    pub fn send(&self, message: M) {

        unsafe { host::api::message::create_data(Tag::none().id(), 0) };

        S::encode(&message).unwrap();

        host::send(self.id);
    }


}

impl<M, S> PartialEq for Process<M, S> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() 
    }
}

impl<M, S> Eq for Process<M, S> {}

impl<M, S> std::hash::Hash for Process<M, S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        
        self.id.hash(state);
    }
}

impl<M, S> std::fmt::Debug for Process<M, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Process")
            .field("id", &self.id())
            .finish()
    }
}

impl<M, S> Clone for Process<M, S> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            serializer_type: self.serializer_type,
        }
    }
}