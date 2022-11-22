use log::LevelFilter;
use once_cell::sync::OnceCell;

static VERBOSITY: OnceCell<LevelFilter> = OnceCell::new();

pub fn verbosity() -> &'static LevelFilter {
    VERBOSITY.get().expect("verbosity is not initialized")
}

pub fn set_global_verbosity(verbosity: LevelFilter) {
    VERBOSITY.set(verbosity).expect("could not set verbosity")
}
