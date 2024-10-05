use std::ops::Not;

use indoc::formatdoc;
use shacl_ast::compiled::component::Nodekind;
use shacl_ast::node_kind::NodeKind;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::helpers::validate_ask_with;
use crate::constraints::helpers::validate_with;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::engine::native::NativeEngine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<S: SRDF + 'static> NativeValidator<S> for Nodekind {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let node_kind = |value_node: &S::Term| {
            match (
                S::term_is_bnode(value_node),
                S::term_is_iri(value_node),
                S::term_is_literal(value_node),
            ) {
                (true, false, false) => matches!(
                    self.node_kind(),
                    NodeKind::BlankNode | NodeKind::BlankNodeOrIri | NodeKind::BlankNodeOrLiteral
                ),
                (false, true, false) => matches!(
                    self.node_kind(),
                    NodeKind::Iri | NodeKind::IRIOrLiteral | NodeKind::BlankNodeOrIri
                ),
                (false, false, true) => matches!(
                    self.node_kind(),
                    NodeKind::Literal | NodeKind::IRIOrLiteral | NodeKind::BlankNodeOrLiteral
                ),
                _ => false,
            }
            .not()
        };

        validate_with(
            store,
            &NativeEngine,
            value_nodes,
            &ValueNodeIteration,
            node_kind,
        )
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for Nodekind {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let node_kind = self.node_kind().clone();

        let query = move |value_node: &S::Term| {
            if S::term_is_iri(value_node) {
                formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:IRI, sh:BlankNodeOrIRI, sh:IRIOrLiteral ) ) }}
                    ",node_kind
                }
            } else if S::term_is_bnode(value_node) {
                formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:Literal, sh:BlankNodeOrLiteral, sh:IRIOrLiteral ) ) }}
                    ", node_kind
                }
            } else {
                formatdoc! {"
                        PREFIX sh: <http://www.w3.org/ns/shacl#>
                        ASK {{ FILTER ({} IN ( sh:BlankNode, sh:BlankNodeOrIRI, sh:BlankNodeOrLiteral ) ) }}
                    ", node_kind
                }
            }
        };

        validate_ask_with(store, value_nodes, query)
    }
}
