use crate::daemon::backend::FindexBackend;
use dbus_crossroads::{Crossroads, IfaceToken};

pub fn get_result(crossroads: &mut Crossroads, mut backend: FindexBackend) -> IfaceToken<()> {
    crossroads.register("org.findex.daemon.query", |builder| {
        builder.method(
            "get_result",
            ("query",),
            ("result",),
            move |_, _, (query,): (String,)| {
                let result = backend.process_query(&query);

                Ok((result,))
            },
        );
    })
}

pub fn get_all(crossroads: &mut Crossroads, mut backend: FindexBackend) -> IfaceToken<()> {
    crossroads.register("org.findex.daemon.query", |builder| {
        builder.method(
            "get_all",
            (),
            ("result",),
            move |_, _, ()| {
                let result = backend.get_all();

                Ok((result,))
            },
        );
    })
}
