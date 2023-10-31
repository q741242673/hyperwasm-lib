use crate::{tag::Tag, error::HperwasmError, module::{params_to_vec, Param}, ProcessConfig};

pub mod api;

pub(crate) fn spawn(
    //node: Option<u64>,
    config: Option<&ProcessConfig>,
    link: Option<Tag>,
    entry: fn(usize),
    arg: usize,
) -> Result<u64, HperwasmError> {
    let entry = entry as usize ;
    
    let params = params_to_vec(&[Param::I32(entry as i32), Param::I32(arg as i32)]);
    let mut id = 0;
    let func = "_lunatic_spawn_by_index";
    let link = match link {
        Some(tag) => tag.id(),
        None => 0,
    };
    let config_id = config.map_or_else(|| ProcessConfig::inherit().id(), |config| config.id());
    
    let result = unsafe {
            println!("happen spawn {:?}", config_id);
            api::process::spawn(
                link,
                config_id,
                -1,
                func.as_ptr(),
                func.len(),
                params.as_ptr(),
                params.len(),
                &mut id,
            )
        };
        

    if result == 0 {
        println!("spawn finish {:?}", config_id);
        Ok(id)
    } else {
        
        Err(HperwasmError::from(id))
    }
}

pub fn process_id() -> u64 {
    unsafe { api::process::process_id() }
}

// pub fn node_id() -> u64 {
//     unsafe { api::distributed::node_id() }
// }

pub fn send(process_id: u64) {

        unsafe { api::message::send(process_id) }

}

#[export_name = "_lunatic_spawn_by_index"]
extern "C" fn _lunatic_spawn_by_index(function: usize, arg: usize) {
    
    let function: fn(usize) = unsafe { std::mem::transmute(function ) };
    function(arg);
}
