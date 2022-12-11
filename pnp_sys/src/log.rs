use ctr::Logger;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOGGER: Logger = Logger::new("/pnp-logs.txt");
}

pub fn error(text: &str) {
    LOGGER.error(text)
}
