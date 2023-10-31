use hyperwasm::{spawn, Process, ProcessConfig, Mailbox};

fn main() {
    println!("hello");
    let mut config = ProcessConfig::new().unwrap();
    config.set_name("child 1");
    config.set_expected_time(4);
    config.set_relative_ddl(6);

    Process::spawn_config(&config, (), |_, _: Mailbox<()>| {
        let a = 3;
        let b = 4;
        let c = a + b;
        println!("hello {}", c);
    });

    let mut config_two = ProcessConfig::new().unwrap();
    config_two.set_name("child 2");
    config_two.set_expected_time(4);
    config_two.set_relative_ddl(6);
    spawn!(&config_two, || {
        let a = 10;
        let b = 4;
        let c = a + b;
        println!("hello {}", c);
    });
}