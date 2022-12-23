use std::rc::Rc;
use yew::prelude::*;

use crate::app::ConfigurationPEq;

#[hook]
pub fn use_configuration() -> Option<Rc<ConfigurationPEq>> {
    use_context::<Rc<ConfigurationPEq>>()
}
