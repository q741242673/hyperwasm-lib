
#[doc(hidden)]
#[macro_export]
macro_rules! spawn_link_config {
    () => {
        hyperwasm::Process::spawn
    };
    (@link) => {
        hyperwasm::Process::spawn_link
    };
    ($config:ident) => {
        hyperwasm::Process::spawn_config
    };
    (@link $config:ident) => {
        hyperwasm::Process::spawn_link_config
    };
}


#[macro_export]
macro_rules! spawn {
    // A background process (no mailbox & not capturing any variables).
    ($(&$config:ident,)? || $body:expr) => {
        hyperwasm::spawn_link_config!($($config)?) ($(&$config,)? (), |_, _: hyperwasm::Mailbox<()>| $body)
    };
    // A background process (no mailbox) that can capture one or more variables.
    ($(&$config:ident,)? |$($argument:ident $(= $value:tt)? ),*| $body:expr) => {
        {
            // Re-assign variables if value is passed to the function
            $($(let $argument = $value)?;)*
            hyperwasm::spawn_link_config!($($config)?) (
                $(&$config,)?
                ($($argument),*),
                |($($argument),*), _: hyperwasm::Mailbox<()>| $body
            )
        }
    };
    ($(&$config:ident,)? |$($argument:ident $(= $value:block)? ),*| $body:expr) => {
        {
            // Re-assign variables if value is passed to the function
            $($(let $argument = $value)?;)*
            hyperwasm::spawn_link_config!($($config)?) (
                $(&$config,)?
                ($($argument),*),
                |($($argument),*), _: hyperwasm::Mailbox<()>| $body
            )
        }
    };
    // A process with a mailbox that is not capturing any variables.
    ($(&$config:ident,)? |$mailbox:ident : Mailbox<$mailbox_ty:ty $( , $mailbox_s:ty )?>| $body:expr) => {
        hyperwasm::spawn_link_config!($($config)?) (
            $(&$config,)?
            (),
            |_, $mailbox: hyperwasm::Mailbox<$mailbox_ty $( , $mailbox_s )?>| $body
        )
    };
    // A process capturing variable `$argument`.
    ($(&$config:ident,)? |$argument:ident, $mailbox:ident : Mailbox<$mailbox_ty:ty $( , $mailbox_s:ty )?>| $body:expr) => {
        hyperwasm::spawn_link_config!($($config)?) (
            $(&$config,)?
            $argument,
            |$argument, $mailbox: hyperwasm::Mailbox<$mailbox_ty $( , $mailbox_s )?>| $body,
        )
    };
}


#[macro_export]
macro_rules! spawn_link {
    // From closure

    // A background process (no mailbox & not capturing any variables).
    ($(&$config:ident,)? || $body:expr) => {
        hyperwasm::spawn_link_config!(@link $($config)?) (
            $(&$config,)?
            (),
            |_, _: hyperwasm::Mailbox<()>| $body
        )
    };
    // A background process (no mailbox) that can capture one or more variables.
    ($(&$config:ident,)? |$($argument:ident $(= $value:tt)? ),*| $body:expr) => {
        {
            // Re-assign variables if value is passed to the function
            $($(let $argument = $value)?;)*
            hyperwasm::spawn_link_config!(@link $($config)?) (
                $(&$config,)?
                ($($argument),*),
                |($($argument),*), _: hyperwasm::Mailbox<()>| $body
            )
        }
    };
    ($(&$config:ident,)? |$($argument:ident $(= $value:block)? ),*| $body:expr) => {
        {
            // Re-assign variables if value is passed to the function
            $($(let $argument = $value)?;)*
            hyperwasm::spawn_link_config!(@link $($config)?) (
                $(&$config,)?
                ($($argument),*),
                |($($argument),*), _: hyperwasm::Mailbox<()>| $body
            )
        }
    };
    // A process with a mailbox that is not capturing any variables.
    ($(&$config:ident,)? |$mailbox:ident : Mailbox<$mailbox_ty:ty $( , $mailbox_s:ty )?>| $body:expr) => {
        hyperwasm::spawn_link_config!(@link $($config)?) (
            $(&$config,)?
            (),
            |_, $mailbox: hyperwasm::Mailbox<$mailbox_ty $( , $mailbox_s )?>| $body
        )
    };
    // A process with a mailbox capturing variable `$argument`.
    ($(&$config:ident,)? |$argument:ident, $mailbox:ident : Mailbox<$mailbox_ty:ty $( , $mailbox_s:ty )?>| $body:expr) => {
        hyperwasm::spawn_link_config!(@link $($config)?) (
            $(&$config,)?
            $argument,
            |$argument, $mailbox: hyperwasm::Mailbox<$mailbox_ty $( , $mailbox_s )?>| $body,
        )
    };

     // A @task that is not capturing any variables.
     (@task $(&$config:ident,)? || $body:expr) => {
        hyperwasm::spawn_link_config!(@link $($config)?) (
            $(&$config,)?
            (),
            |_, protocol: hyperwasm::protocol::Protocol<hyperwasm::protocol::Send<_,hyperwasm::protocol::TaskEnd>>| {
                let _ = protocol.send($body);
            },
        )
    };
    // A @task capturing variables.
    (@task $(&$config:ident,)? |$($argument:ident $(= $value:block)? ),*| $body:expr) => {
        {
            // Re-assign variables if value is passed to the function
            $($(let $argument = $value)?;)*
            hyperwasm::spawn_link_config!(@link $($config)?) (
                $(&$config,)?
                ($($argument),*),
                |($($argument),*), protocol: hyperwasm::protocol::Protocol<
                        hyperwasm::protocol::Send<_,hyperwasm::protocol::TaskEnd>>| {
                    let _ = protocol.send($body);
                },
            )
        }
    };
    (@task $(&$config:ident,)? |$($argument:ident $(= $value:tt)? ),*| $body:expr) => {
        {
            // Re-assign variables if value is passed to the function
            $($(let $argument = $value)?;)*
            hyperwasm::spawn_link_config!(@link $($config)?) (
                $(&$config,)?
                ($($argument),*),
                |($($argument),*), protocol: hyperwasm::protocol::Protocol<
                        hyperwasm::protocol::Send<_,hyperwasm::protocol::TaskEnd>>| {
                    let _ = protocol.send($body);
                },
            )
        }
    };

    // A protocol that is not capturing any variables.
    ($(&$config:ident,)? |$protocol:ident : Protocol<$proto_ty:ty>| $body:expr) => {
        hyperwasm::spawn_link_config!(@link $($config)?) (
            $(&$config,)?
            (),
            |_, $protocol: hyperwasm::protocol::Protocol<$proto_ty>| $body,
        )
    };
    // A protocol capturing variable `$argument`.
    ($(&$config:ident,)? |$argument:ident, $protocol:ident : Protocol<$proto_ty:ty>| $body:expr) => {
        hyperwasm::spawn_link_config!(@link $($config)?) (
            $(&$config,)?
            $argument,
            |$argument, $protocol: hyperwasm::protocol::Protocol<$proto_ty>| $body,
        )
    };
}
