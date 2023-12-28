use prefixmap::IriRef;
use srdf::{RDFNode, numeric_literal::NumericLiteral};

use crate::{component::Component, target::Target, message_map::MessageMap, severity::Severity};

#[derive(Debug, Clone)]
pub struct NodeShape {
    id: RDFNode,
    components: Vec<Component>,
    targets: Vec<Target>,
    property_shapes: Vec<RDFNode>,
    closed: bool,
    ignored_properties: Vec<IriRef>,
    deactivated: bool,
    message: MessageMap,
    severity: Option<Severity>,
    name: MessageMap,
    description: MessageMap,

    // SHACL spec says that the values of sh:order should be decimals but in the examples they use integers. `NumericLiteral` also includes doubles.
    order: Option<NumericLiteral>, 

    group: Option<RDFNode>,
    source_iri: Option<IriRef>
}

impl NodeShape {

    pub fn new(id: RDFNode) -> Self {
        NodeShape {
         id, 
         components: Vec::new(),
         targets: Vec::new(),
         property_shapes: Vec::new(),
         closed: false,
         ignored_properties: Vec::new(),
         deactivated: false,
         message: MessageMap::new(),
         severity: None,
         name: MessageMap::new(),
         description: MessageMap::new(),
         order: None, 
         group: None,
         source_iri: None
       }
    }
}