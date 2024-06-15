use std::collections::HashMap;
use crate::config::ServiceCode;

type ServiceHistory = HashMap<String, Vec<ServiceCode>>;
pub type BaseHistory = HashMap<String, ServiceHistory>;
