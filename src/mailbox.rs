use std::{marker::PhantomData, time::Duration, fmt};

use crate::{serializer::{Bincode, Serializer, DecodeError}, function::process::{Process, NoLink, IntoProcess}, host::{self, api::message}, tag::Tag, ProcessConfig};

const LINK_DIED: u32 = 1;
const TIMEOUT: u32 = 9027;

pub struct Catching;

pub struct Mailbox<M, S = Bincode, L = ()>
where
    S: Serializer<M>,
{
    phantom: PhantomData<(M, S, L)>,
}


impl<M, S> Mailbox<M, S, ()>
where
    S: Serializer<M>,
{

    #[track_caller]
    pub fn receive(&self) -> M {
        self.receive_(&[], None).unwrap()
    }

 
    #[track_caller]
    pub fn tag_receive(&self, tags: &[Tag]) -> M {
        self.receive_(tags, None).unwrap()
    }


    pub fn catch_link_failure(self) -> Mailbox<M, S, Catching> {
        unsafe {
            host::api::process::die_when_link_dies(0);
            Mailbox::<M, S, Catching>::new()
        }
    }
}

impl<M, S, L> Mailbox<M, S, L>
where
    S: Serializer<M>,
{

    pub fn this(&self) -> Process<M, S> {
        Process::new(host::process_id())
    }

   
    pub fn try_receive(&self, timeout: Duration) -> MailboxResult<M> {
        self.receive_(&[], Some(timeout))
    }

   
    pub fn receive_timeout(&self, timeout: Duration) -> MailboxResult<M> {
        self.receive_(&[], Some(timeout))
    }


    pub fn tag_receive_timeout(&self, tags: &[Tag], timeout: Duration) -> MailboxResult<M> {
        self.receive_(tags, Some(timeout))
    }

    fn receive_(&self, tags: &[Tag], timeout: Option<Duration>) -> MailboxResult<M> {
        let tags: Vec<i64> = tags.iter().map(|tag| tag.id()).collect();
        let timeout_ms = match timeout {
            Some(timeout) => timeout.as_millis() as u64,
            None => u64::MAX,
        };
        let message_type = unsafe { message::receive(tags.as_ptr(), tags.len(), timeout_ms) };
        match message_type {
            LINK_DIED => MailboxResult::LinkDied(unsafe { Tag::from(message::get_tag()) }),
            TIMEOUT => MailboxResult::TimedOut,
            _ => match S::decode() {
                Ok(msg) => MailboxResult::Message(msg),
                Err(err) => MailboxResult::DeserializationFailed(err),
            },
        }
    }


    pub unsafe fn new() -> Self {
        Self {
            phantom: PhantomData {},
        }
    }
}

impl<M, S> Mailbox<M, S, Catching>
where
    S: Serializer<M>,
{

 
    pub fn receive(&self) -> MailboxResult<M> {
        self.receive_(&[], None)
    }


    pub fn tag_receive(&self, tags: &[Tag]) -> MailboxResult<M> {
        self.receive_(tags, None)
    }
}

impl<M, S, L> Clone for Mailbox<M, S, L>
where
    S: Serializer<M>,
{
    fn clone(&self) -> Self {
        Self {
            phantom: self.phantom,
        }
    }
}

impl<M, S, L> Copy for Mailbox<M, S, L> where S: Serializer<M> {}

impl<M, S, L> fmt::Debug for Mailbox<M, S, L>
where
    S: Serializer<M>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mailbox")
            .field("message", &std::any::type_name::<M>())
            .field("serializer", &std::any::type_name::<S>())
            .field("link", &std::any::type_name::<L>())
            .finish()
    }
}


#[derive(Debug)]
pub enum MailboxResult<T> {
    Message(T),
    DeserializationFailed(DecodeError),
    TimedOut,
    LinkDied(Tag),
}

impl<T> MailboxResult<T> {
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            MailboxResult::Message(msg) => msg,
            MailboxResult::DeserializationFailed(err) => panic!("{:?}", err),
            MailboxResult::TimedOut => panic!("TimedOut"),
            MailboxResult::LinkDied(_) => panic!("LinkDied"),
        }
    }


    pub fn is_message(&self) -> bool {
        matches!(self, MailboxResult::Message(_))
    }


    pub fn is_link_died(&self) -> bool {
        matches!(self, MailboxResult::LinkDied(_))
    }

    pub fn is_timed_out(&self) -> bool {
        matches!(self, MailboxResult::TimedOut)
    }
}

impl<M, S> NoLink for Mailbox<M, S> where S: Serializer<M> {}


impl<M, S> IntoProcess<M, S> for Mailbox<M, S>
where
    S: Serializer<M>,
{
    type Process = Process<M, S>;

    fn spawn<C>(
        capture: C,
        entry: fn(C, Self),
        link: Option<Tag>,
        config: Option<&ProcessConfig>,
        //node: Option<u64>,
    ) -> Self::Process
    where
        S: Serializer<C> + Serializer<M>,
    {
        let entry = entry as usize ;
        //let node_id = node.unwrap_or_else(host::node_id);

  
        match host::spawn( config, link, type_helper_wrapper::<C, M, S>, entry) {
            Ok(id) => {
                // If the captured variable is of size 0, we don't need to send it to another
                // process.
                // if std::mem::size_of::<C>() == 0 {
                    
                    Process::new( id)
                    
                // } else {
                //     let child = Process::<C, S>::new(id);
                //     child.send(capture);

                //     unsafe { std::mem::transmute(child) }
                // }
            }
            Err(err) => panic!("Failed to spawn a process: {}", err),
        }
    }
}


fn type_helper_wrapper<C, M, S>(function: usize)
where
    S: Serializer<C> + Serializer<M>,
{
    
    let captured = //if std::mem::size_of::<C>() == 0 {
        unsafe { std::mem::MaybeUninit::<C>::zeroed().assume_init() };
    // } else {
    //     unsafe { Mailbox::<C, S>::new() }.receive()
    // };
    let mailbox = unsafe { Mailbox::new() };
    let function: fn(C, Mailbox<M, S>) = unsafe { std::mem::transmute(function ) };
    function(captured, mailbox);
}
