use indoc::formatdoc;
use shacl_ast::compiled::component::Class;
use srdf::QuerySRDF;
use srdf::RDFS_SUBCLASS_OF;
use srdf::RDF_TYPE;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::helpers::validate_ask_with;
use crate::constraints::helpers::validate_with;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::engine::native::NativeEngine;
use crate::helper::srdf::get_objects_for;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<S: SRDF + 'static> NativeValidator<S> for Class<S> {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let class = |value_node: &S::Term| {
            if S::term_is_literal(value_node) {
                return true;
            }

            let is_class_valid = get_objects_for(store, value_node, &S::iri_s2iri(&RDF_TYPE))
                .unwrap_or_default()
                .iter()
                .any(|ctype| {
                    ctype == self.class_rule()
                        || get_objects_for(store, ctype, &S::iri_s2iri(&RDFS_SUBCLASS_OF))
                            .unwrap_or_default()
                            .contains(self.class_rule())
                });

            !is_class_valid
        };

        validate_with(
            store,
            &NativeEngine,
            value_nodes,
            &ValueNodeIteration,
            class,
        )
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for Class<S> {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let class_value = self.class_rule().clone();

        let query = move |value_node: &S::Term| {
            formatdoc! {"
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
            ASK {{ {} rdf:type/rdfs:subClassOf* {} }}
        ", value_node, class_value,
            }
        };

        validate_ask_with(store, value_nodes, query)
    }
}
