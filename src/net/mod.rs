mod methods;

#[cfg(test)]
mod integration_test;


use crate::configuration::Config;

struct ConnectionManager {
    config: Box<dyn Config>
}
