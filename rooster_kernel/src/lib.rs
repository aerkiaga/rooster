//! This crate contains **Rooster**'s proof kernel code,
//! an implementation of the [Calculus of Constructions](https://en.wikipedia.org/wiki/Calculus_of_constructions) (CoC)
//! with inductive types.
//!
//! CoC is a mathematical type theory and programming language
//! developed by adding dependent types, polymorphism and
//! type operators to simply typed lambda calculus. Devised by
//! Thierry Coquand, it is _strongly normalizing_ (i.e. every
//! program in CoC terminates, and thus CoC is not
//! Turing-complete) and follows the Curry-Howard isomorphism,
//! which links together programs and proofs.
//!
//! CoC can be used to write both standard programs and
//! proofs of propositions. It provides _intuitionistic_
//! logic as a foundation to build upon, which is weaker
//! than classical logic.
//!
//! A _term_ in CoC can take the following forms:
//! * x, where 'x' is an identifier.
//! * A B, for terms A and B.
//! * λx:A.B, for identifier 'x' and terms A and B.
//! * ∀x:A.B, for identifier 'x' and terms A and B.
//! * 𝐘x:A.B, for identifier 'x' and terms A and B.
//! * ℺A.B, for terms A and B.
//!
//! _Set_, _Prop_, _Type_ and _TypeN_ (for every non-negative integer N)
//! are special, pre-defined identifiers.
//!
//! The kernel offers two main operations on terms. The first
//! is _normalizing_, which is equivalent to computation and
//! reduces terms to an equivalent form. The second is
//! _type inference_, which determines the type of a term,
//! but also checks if the term itself is valid. The type of
//! a term is also a term.
//!
//! Finally, the kernel offers the ability to _define_
//! variables within a _state_. A definition like
//! `x = B: A` involves normalizing both A and B,
//! checking that A is valid, and comparing B's type
//! to A. Then, the new term is added to the state.
//!
//! ## Free variables
//! A term is said to freely contain a variable
//! if an occurrence of it is anywhere within the term
//! and it is not bound to any inner scope. For example,
//! x a x contains 'x' as a free variable, while
//! λx.x a x doesn't. However, (λx.x a x) x does contain 'x'
//! (the last occurrence). Same goes for terms involving
//! ∀ or 𝐘 instead of λ.
//!
//! ## Substitution
//! Substitution involves replacing a variable within
//! a term by a second term. All free occurrences of the
//! variable are replaced.
//!
//! There's a special case to consider. If the new term
//! contains free variables that would become bound upon
//! substitution, renaming is performed. For example,
//! consider the term λx:A.y x, where 'y' is to be
//! replaced by a term containing a free occurrence of 'x'.
//! Substituting it into the inner scope would bind 'x',
//! so it would refer to a different variable than it should.
//! This is solved by renaming the inner 'x', like so:
//! λx0:A.y x0. Now, substitution can be performed.
//!
//! To rename a variable like this, a name is chosen that
//! is not a free variable within the abstraction.
//! For example, in λx:A.x x0, 'x' wouldn't be renamed to
//! 'x0', a different name would be chosen.
//!
//! Additionally, the new variable name must not occur in
//! the new term. For example, if 'x' is to be replaced with
//! x x0 (where x is an outer variable) in λx:A.x, then 'x'
//! wouldn't be renamed to 'x0' either.
//!
//! Finally, the new name must not coincide with the variable
//! to be replaced. E.g., to replace 'x0' with x in λx:A.x0,
//! the variable 'x' can't be renamed to 'x0'.
//!
//! Then, substitution is used to replace the variable.
//! This means that all of the above is applied again.
//!
//! ## Normalization rules
//! The following rules are applied, whenever valid, in any
//! order, until none of them can be applied further. Since
//! CoC is strongly normalizing, every finite term is fully
//! normalized after a finite amount of steps.
//!
//! ### β-reduction
//! Transforms (λx:T.F) P into F[x:=P], i.e. "returns" F,
//! with every occurrence of 'x' replaced with term P.
//!
//! Substitution is used to perform this replacement,
//! with the rules explained above.
//!
//! ### η-reduction
//! Transforms λx:T.F x into F only if 'x' doesn't
//! appear free (i.e. accounting for shadowing) in F.
//!
//! ### δ-reduction
//! Replaces each free variable that's defined in the
//! containing state by its value, using substitution.
//!
//! ### α-conversion
//! This involves renaming identifiers. It can be performed
//! trivially for any expression containing identifiers,
//! so it is not necessary to perform it for normalizing,
//! unless two terms need to be compared (since they need
//! to have the exact same variable names).
//!
//! Renaming identifiers is also done via substitution,
//! so all of the rules of substitution apply here.
//!
//! In this kernel, α-conversion is only aimed at comparing
//! terms. The way it is performed is the following:
//! every variable defined within the term is given a name
//! of the form '#n', where n is some natural number.
//! Two terms that are equivalent up to α-conversion will
//! always become identical after applying this to both.
//!
//! ### Fixed point reduction
//! An expression of the form 𝐘x:A.B will be replaced by
//! (λx:A.B)(𝐘x:A.B). Note that this operation can succeed
//! every time, _ad infinitum_, so the expression must be
//! normalized before every fixed point reduction. If the
//! term is valid (which should have been checked), type
//! rules will enforce eventual termination.
//!
//! Fixed point reduction may be restricted to only operate
//! if B is a term of the form λy:C.D, and otherwise do nothing.
//!
//! ## Type inference rules
//! * Variables defined outside the term (in its containing
//! _state_ or as variables in a term enclosing it) take the type
//! they were defined with.
//! * _Set_ and _Prop_ have type _Type_, _Type_ has type _Type0_
//! and every _TypeN_ has type _TypeM_, with M = N + 1.
//! * A B uses the following rules:
//!   - If A has type 𝐘x:S.W, its type is fixed point reduced
//!     once and then normalized. The remaining type must be of the
//!     form ∀x:T.V.
//!   - If A is a term of the form C D, where the type of C is
//!     an **inductive type**, rules for **match terms** apply.
//!   - Otherwise, if B is of type T (being A of type ∀x:T.V),
//!     the term takes as type V with 'x' replaced by B.
//!   - If none of the conditions apply, the term is invalid.
//! * λx:A.B takes type ∀x:A.T, with T being the type of B
//! with each free occurrence of 'x' within B taking type A.
//!   - A's type's type must be _Type_ or _TypeN_.
//!   - If 'x' is captured by an outer scope or _state_, it is renamed.
//! * ∀x:A.B takes the type of B, with each free occurrence
//! of 'x' within B taking type A.
//!   - A's type's type must be _Type_ or _TypeN_,
//!     and the output type's type must be _Type_ or _TypeN_.
//!   - If 'x' is captured by an outer scope or _state_, it is renamed.
//! * 𝐘x:A.B takes type A, provided that B is also of type A.
//!   Additionally, either of the following rules must apply:
//!   - First option, B, and thus 𝐘x:A.B, fulfills the rules for **inductive types**.
//!   - Second option, 𝐘x:A.B fulfills the rules for **primitive recursive functions**.
//!   - Note that if 'x' is captured by an outer scope or _state_, it is renamed.
//! * ℺A.B takes type A, provided that a single fixed point expansion
//!   of A yields the type of B.
//!
//! ### Match terms
//! A match term is an expression of the form C D B,
//! wherein C has a type that is an **inductive type**.
//!
//! Regular type rules are applied, except when C is an
//! identifier and B contains A. Then, each occurrence of C
//! is replaced by a different **inductive instance** when
//! inferring the type of C D B.
//!
//! If the type of C D is of the form ∀T:R.∀x1:X1.∀x2:X2. ... T,
//! then that occurrence of 'T' is replaced by B without
//! modifying occurrences of C within B.
//!
//! If any of those Xi sub-terms contain 'T', it will occur
//! as ∀y1:Y1.∀y2:Y2. ... T, due to the requirements for
//! **inductive types**. Then, before replacing 'T' with B,
//! every occurrence of C within B will be replaced by
//! λR:Type.λT:R.℺G.λx1:X1'.λx2:X2'. ... Q, where every Xj' equals
//! its corresponding Xj after replacing ∀ expressions with λ,
//! and Q equals xi y1 y2 ... (note the i index from the
//! outer expression, xi from the inner term and yi from the
//! outer term). G is the type of C.
//!
//! ### Inductive types
//! An inductive type has the form ∀R:W.∀T:R.M, or alternatively
//! 𝐘S:Set.∀R:W.∀T:R.M (i.e., the first form enclosed in a
//! fixed point operator).
//!
//! Rules for type-checking are the following:
//! * 'T' only appears _strictly positively_ within M.
//! * 'S' only appears _strictly positively_ within each
//!   parameter type of M.
//!
//! ∀R:W.∀T:R.M has type Set if it fulfills the criteria
//! above. 𝐘S:Set.B has type Set if B also has type Set,
//! otherwise fails to check.
//!
//! ### Primitive recursive functions
//! These are objects of the form 𝐘s:T.F, wherein
//! F is not an **inductive type**.
//!
//! Being F of the form λx1:A1.λx2:A2. ... D, there
//! must be some i such that Ai is an **inductive
//! type** and 's' does not occur anywhere within D
//! _except_ when fulfilling the condition below.
//!
//! If a sub-expression within D is of the form
//! xi P1 P2 P3 ..., and any such Pj is itself
//! of the form λy1:B1.λy2:B2. ... E, then E
//! can contain an expression s Q1 Q2 Q3 ...,
//! provided that Qi (note the i index from above)
//! is exactly one of the variables in y1, y2, y3 ...

/// A type containing extra information for debugging proofs.
///
/// In principle, bugs in code manipulating this type
/// should not affect proof checking, so this code is
/// outside the trusted base.
///
#[derive(Clone, Debug)]
pub enum TermDebugContext {
    /// No information, don't propagate.
    Ignore,
    /// The term originates from a span of the input source code.
    CodeSpan((usize, usize)),
    /// The term is the type of a term with a given debug context.
    TypeOf(Box<TermDebugContext>),
}

impl TermDebugContext {
    /// Get the span of source code containing this term.
    pub fn get_span(self: &Self) -> (usize, usize) {
        match self {
            Self::Ignore => (0, 0),
            Self::CodeSpan(s) => *s,
            Self::TypeOf(b) => b.get_span(),
        }
    }
}

/// A (valid or invalid) term in Calculus of Constructions.
#[derive(Clone)]
pub enum Term {
    /// A symbol representing a bound variable,
    /// a different term in the current state,
    /// or the special identifiers `Prop` and `Type(1)`.
    Identifier(String, TermDebugContext),
    /// Application of an object onto another,
    /// i.e. A B, for any terms A and B.
    Application {
        function_term: Box<Term>,
        parameter_term: Box<Term>,
        debug_context: TermDebugContext,
    },
    /// Typed lambda abstraction, i.e. λx:A.B,
    /// for any identifier x and terms A and B.
    Lambda {
        binding_identifier: String,
        binding_type: Box<Term>,
        value_term: Box<Term>,
        debug_context: TermDebugContext,
    },
    /// Typed for all ... expression, i.e. ∀x:A.B,
    /// for any identifier x and terms A and B.
    Forall {
        binding_identifier: String,
        binding_type: Box<Term>,
        value_term: Box<Term>,
        debug_context: TermDebugContext,
    },
    /// Inductive Y combinator-like, i.e. 𝐘x:A.B,
    /// for any identifier x and terms A and B.
    FixedPoint {
        binding_identifier: String,
        binding_type: Box<Term>,
        value_term: Box<Term>,
        debug_context: TermDebugContext,
    },
    /// Constructor return value, i.e. ℺A.B,
    /// for any terms A and B.
    Constructor {
        type_term: Box<Term>,
        value_term: Box<Term>,
        debug_context: TermDebugContext,
    },
}

impl PartialEq for Term {
    /// Two terms are considered equal if and only if
    /// they contain identical syntactic trees, with
    /// every variable named exactly the same across them.
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Identifier(s, _) => {
                if let Self::Identifier(s2, _) = other {
                    s == s2
                } else {
                    false
                }
            }
            Self::Application {
                function_term,
                parameter_term,
                debug_context: _,
            } => {
                if let Self::Application {
                    function_term: function_term2,
                    parameter_term: parameter_term2,
                    debug_context: _,
                } = other
                {
                    function_term == function_term2 && parameter_term == parameter_term2
                } else {
                    false
                }
            }
            Self::Lambda {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                if let Self::Lambda {
                    binding_identifier: binding_identifier2,
                    binding_type: binding_type2,
                    value_term: value_term2,
                    debug_context: _,
                } = other
                {
                    binding_identifier == binding_identifier2
                        && binding_type == binding_type2
                        && value_term == value_term2
                } else {
                    false
                }
            }
            Self::Forall {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                if let Self::Forall {
                    binding_identifier: binding_identifier2,
                    binding_type: binding_type2,
                    value_term: value_term2,
                    debug_context: _,
                } = other
                {
                    binding_identifier == binding_identifier2
                        && binding_type == binding_type2
                        && value_term == value_term2
                } else {
                    false
                }
            }
            Self::FixedPoint {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                if let Self::FixedPoint {
                    binding_identifier: binding_identifier2,
                    binding_type: binding_type2,
                    value_term: value_term2,
                    debug_context: _,
                } = other
                {
                    binding_identifier == binding_identifier2
                        && binding_type == binding_type2
                        && value_term == value_term2
                } else {
                    false
                }
            }
            Self::Constructor {
                type_term,
                value_term,
                debug_context: _,
            } => {
                if let Self::Constructor {
                    type_term: type_term2,
                    value_term: value_term2,
                    debug_context: _,
                } = other
                {
                    type_term == type_term2 && value_term == value_term2
                } else {
                    false
                }
            }
        }
    }
}

impl Eq for Term {}

/// A type representing an error encountered by the kernel while executing some input.
#[derive(Debug)]
pub enum KernelError {
    /// A term contains some identifier that hasn't been defined before.
    UndefinedIdentifier {
        identifier: String,
        identifier_context: TermDebugContext,
    },
    /// A term has been applied on some argument of a different type than that it accepts.
    NonmatchingArgument {
        expected_type: Term,
        observed_type: Term,
        function_context: TermDebugContext,
        parameter_context: TermDebugContext,
    },
    /// A term that accepts no arguments has been applied on some other term.
    InvalidApplication {
        nonfunction_type: Term,
        nonfunction_context: TermDebugContext,
        parameter_context: TermDebugContext,
    },
    /// A term has been defined, but the stated type doesn't match its actual type.
    NonmatchingDefinition {
        expected_type: Term,
        observed_type: Term,
        signature_context: TermDebugContext,
        definition_context: TermDebugContext,
    },
    /// A term which type should be Set, Prop or Type has a different type.
    InvalidType {
        incorrect_term: Term,
        incorrect_type: Term,
        incorrect_context: TermDebugContext,
    },
    /// A term which type should be Set has a different type.
    InvalidTypeSet {},
    /// An inductive instance's type doesn't match its specified type.
    InvalidInstance {},
    // Either the type definition or one of its parameters don't evaluate
    // to a term of the form A1→A2→A3→...→B.
    MisshapenInductiveDefinition {
        unexpected_subterm: Term,
        subterm_context: TermDebugContext,
        full_term_context: TermDebugContext,
    },
    // An identifier is at a negative position in the type definition,
    // which is not allowed (e.g. (x→B)→C).
    NegativeInductiveDefinition {
        negative_subterm: Term,
        subterm_context: TermDebugContext,
        full_term_context: TermDebugContext,
    },
    // Either the type definition or a recursive function
    // don't evaluate to a term of the form A1→A2→A3→...→B
    // or λx1:A1.λx2:A2.λx3:A3. ... B.
    MisshapenRecursiveDefinition {
        unexpected_subterm: Term,
        subterm_context: TermDebugContext,
        full_term_context: TermDebugContext,
    },
    // The function doesn't have any single parameter
    // that is clearly decreasing for every recursive invocation.
    NonprimitiveRecursiveFunction {
        full_term_context: TermDebugContext,
    },
    // The type of a recursive definition includes
    // the definition itself.
    SelfReferencingRecursiveType {
        full_term_context: TermDebugContext,
    },
}

/// Context that terms inhabit.
pub struct State {
    terms: std::collections::HashMap<String, (Term, Term)>,
}

impl Term {
    fn debug_context(self: &Self) -> &TermDebugContext {
        match self {
            Self::Identifier(_, db) => db,
            Self::Application {
                function_term: _,
                parameter_term: _,
                debug_context,
            } => debug_context,
            Self::Lambda {
                binding_identifier: _,
                binding_type: _,
                value_term: _,
                debug_context,
            }
            | Self::Forall {
                binding_identifier: _,
                binding_type: _,
                value_term: _,
                debug_context,
            }
            | Self::FixedPoint {
                binding_identifier: _,
                binding_type: _,
                value_term: _,
                debug_context,
            } => debug_context,
            Self::Constructor {
                type_term: _,
                value_term: _,
                debug_context,
            } => debug_context,
        }
    }

    fn with_new_debug_context(self: &Self, new_debug_context: &TermDebugContext) -> Self {
        match self {
            Self::Identifier(s, _) => Self::Identifier(s.clone(), new_debug_context.clone()),
            Self::Application {
                function_term,
                parameter_term,
                debug_context: _,
            } => Self::Application {
                function_term: function_term.clone(),
                parameter_term: parameter_term.clone(),
                debug_context: new_debug_context.clone(),
            },
            Self::Lambda {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => Self::Lambda {
                binding_identifier: binding_identifier.clone(),
                binding_type: binding_type.clone(),
                value_term: value_term.clone(),
                debug_context: new_debug_context.clone(),
            },
            Self::Forall {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => Self::Forall {
                binding_identifier: binding_identifier.clone(),
                binding_type: binding_type.clone(),
                value_term: value_term.clone(),
                debug_context: new_debug_context.clone(),
            },
            Self::FixedPoint {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => Self::FixedPoint {
                binding_identifier: binding_identifier.clone(),
                binding_type: binding_type.clone(),
                value_term: value_term.clone(),
                debug_context: new_debug_context.clone(),
            },
            Self::Constructor {
                type_term,
                value_term,
                debug_context: _,
            } => Self::Constructor {
                type_term: type_term.clone(),
                value_term: value_term.clone(),
                debug_context: new_debug_context.clone(),
            },
        }
    }

    // Checks if the given term contains an identifier in free form,
    /// that is, not shadowed by any variable defined within the term.
    pub fn contains(self: &Self, identifier: &String) -> bool {
        match self {
            Self::Identifier(s, _) => s == identifier,
            Self::Application {
                function_term,
                parameter_term,
                debug_context: _,
            } => function_term.contains(identifier) || parameter_term.contains(identifier),
            Self::Lambda {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::Forall {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::FixedPoint {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                binding_type.contains(identifier)
                    || binding_identifier != identifier && value_term.contains(identifier)
            }
            Self::Constructor {
                type_term,
                value_term,
                debug_context: _,
            } => type_term.contains(identifier) || value_term.contains(identifier),
        }
    }

    /// Replaces every occurrence of a variable by a new term.
    ///
    /// There are two special cases to consider. Firstly, shadowing,
    /// so that replacing 'x' by V in (λx:A.x B) x gives (λx:A.x B) V,
    /// and the same in (∀x:A.x B) x produces (∀x:A.x B) V.
    /// Secondly, in a case like λy:A.x y, replacing that 'x' would
    /// produce a wrong result if V happens to contain 'y'. In this case,
    /// the substitution would be applied like this: λy0:A.V y0.
    /// Same goes for ∀y:A.x y. In this case, a new name is chosen
    /// that does not occur freely in the inner expression (here x y,
    /// so y0 is a valid name), and then `replace` is called on it.
    ///
    /// If 'x' needs to be replaced with x x0 in λx:A.x, 'x' won't be
    /// renamed to 'x0'. Same goes for 'x0' to 'x' in λx:A.x0.
    ///
    /// Regarding debug context, this function applies an exception:
    /// if the value to be substituted into a given term has a debug
    /// context TermDebugContext::Ignore, the debug context of the
    /// original variable occurrences will be preserved, rather than
    /// copying over the context from the new term.
    ///
    pub fn replace(self: &mut Self, identifier: &String, value: &Self) {
        match self {
            Self::Identifier(s, db) => {
                if s == identifier {
                    if let TermDebugContext::Ignore = value.debug_context() {
                        // ignore new debug context
                        *self = value.with_new_debug_context(db)
                    } else {
                        // copy over new debug context
                        *self = value.clone()
                    }
                }
            }
            Self::Application {
                function_term,
                parameter_term,
                debug_context: _,
            } => {
                function_term.replace(identifier, value);
                parameter_term.replace(identifier, value);
            }
            Self::Lambda {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::Forall {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::FixedPoint {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                binding_type.replace(identifier, value);
                if binding_identifier != identifier {
                    if value.contains(binding_identifier) {
                        let mut suffix: u64 = 0;
                        loop {
                            let new_binding_identifier =
                                format!("{}{}", binding_identifier, suffix);
                            if !value_term.contains(&new_binding_identifier)
                                && !value.contains(&new_binding_identifier)
                                && &new_binding_identifier != identifier
                            {
                                value_term.replace(
                                    binding_identifier,
                                    &Self::Identifier(
                                        new_binding_identifier.clone(),
                                        TermDebugContext::Ignore,
                                    ),
                                );
                                *binding_identifier = new_binding_identifier;
                                break;
                            }
                            if suffix == u64::MAX {
                                panic!("Ran out of suffixes for variable renaming");
                            }
                            suffix += 1;
                        }
                    }
                    value_term.replace(identifier, value);
                }
            }
            Self::Constructor {
                type_term,
                value_term,
                debug_context: _,
            } => {
                type_term.replace(identifier, value);
                value_term.replace(identifier, value);
            }
        }
    }

    /// Applies the β-reduction rule of lambda calculus.
    /// If the term can be β-reduced, one such reduction is performed
    /// and this method returns true. Otherwise, it returns false
    /// and no change is performed.
    ///
    /// (λx:T.F) P = F[x:=P]
    ///
    /// Please note that no type checking is performed.
    ///
    pub fn beta_reduce(self: &mut Self) -> bool {
        if let Self::Application {
            function_term,
            parameter_term,
            debug_context,
        } = self
        {
            let new_debug_context = debug_context;
            if let Self::Lambda {
                binding_identifier,
                binding_type: _,
                value_term,
                debug_context: _,
            } = &mut **function_term
            {
                value_term.replace(&binding_identifier, parameter_term);
                *self = (**value_term).with_new_debug_context(new_debug_context);
                return true;
            }
        }
        match self {
            Self::Identifier(_, _) => false,
            Self::Application {
                function_term,
                parameter_term,
                debug_context: _,
            } => function_term.beta_reduce() || parameter_term.beta_reduce(),
            Self::Lambda {
                binding_identifier: _,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::Forall {
                binding_identifier: _,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::FixedPoint {
                binding_identifier: _,
                binding_type,
                value_term,
                debug_context: _,
            } => binding_type.beta_reduce() || value_term.beta_reduce(),
            Self::Constructor {
                type_term,
                value_term,
                debug_context: _,
            } => type_term.beta_reduce() || value_term.beta_reduce(),
        }
    }

    /// Applies the η-reduction rule of lambda calculus.
    /// If the term can be η-reduced, one such reduction is performed
    /// and this method returns true. Otherwise, it returns false
    /// and no change is performed.
    ///
    /// λx:T.F x = F only if F[x:=M] = F for all M
    ///
    /// Please note that no type checking is performed.
    ///
    pub fn eta_reduce(self: &mut Self) -> bool {
        if let Self::Lambda {
            binding_identifier,
            binding_type: _,
            value_term,
            debug_context: _,
        } = self
        {
            // do not use the outer debug context, i.e.
            // refer to just F rather than λx:T.F x in diagnostics
            if let Self::Application {
                function_term,
                parameter_term,
                debug_context: _,
            } = &**value_term
            {
                if let Self::Identifier(s, _) = &**parameter_term {
                    if s == binding_identifier && !function_term.contains(s) {
                        *self = *function_term.clone();
                        return true;
                    }
                }
            }
        }
        match self {
            Self::Identifier(_, _) => false,
            Self::Application {
                function_term,
                parameter_term,
                debug_context: _,
            } => function_term.eta_reduce() || parameter_term.eta_reduce(),
            Self::Lambda {
                binding_identifier: _,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::Forall {
                binding_identifier: _,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::FixedPoint {
                binding_identifier: _,
                binding_type,
                value_term,
                debug_context: _,
            } => binding_type.eta_reduce() || value_term.eta_reduce(),
            Self::Constructor {
                type_term,
                value_term,
                debug_context: _,
            } => type_term.eta_reduce() || value_term.eta_reduce(),
        }
    }

    /// Applies both β- and η-reduction until fully normalized.
    /// Preserves variable names.
    ///
    /// Please note that no type checking is performed.
    ///
    pub fn normalize(self: &mut Self) {
        loop {
            if !(self.beta_reduce() || self.eta_reduce()) {
                break;
            }
        }
    }

    /// Applies δ-reduction to the greatest extent possible.
    /// Preserves variable names.
    ///
    /// Please note that no type checking is performed.
    ///
    pub fn delta_normalize(self: &mut Self, state: &State) {
        for (name, term) in &state.terms {
            self.replace(name, &term.1);
        }
    }

    fn alpha_normalize_recursive(self: &mut Self, next_suffix: u64) {
        match self {
            Self::Identifier(_, _) => {}
            Self::Application {
                function_term,
                parameter_term,
                debug_context: _,
            } => {
                function_term.alpha_normalize_recursive(next_suffix);
                parameter_term.alpha_normalize_recursive(next_suffix);
            }
            Self::Lambda {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::Forall {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::FixedPoint {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                binding_type.alpha_normalize_recursive(next_suffix);
                let new_binding_identifier = format!("#{}", next_suffix);
                value_term.replace(
                    binding_identifier,
                    &Self::Identifier(new_binding_identifier.clone(), TermDebugContext::Ignore),
                );
                *binding_identifier = new_binding_identifier;
                if next_suffix == u64::MAX {
                    panic!("Ran out of suffixes for variable renaming");
                }
                value_term.alpha_normalize_recursive(next_suffix + 1);
            }
            Self::Constructor {
                type_term,
                value_term,
                debug_context: _,
            } => {
                type_term.alpha_normalize_recursive(next_suffix);
                value_term.alpha_normalize_recursive(next_suffix);
            }
        }
    }

    /// Applies α-conversion, reassigning variable names systematically.
    ///
    /// Two equivalent terms always become identical after applying
    /// `.delta_normalize()`, `.normalize()` and `.alpha_normalize()` on both.
    ///
    pub fn alpha_normalize(self: &mut Self) {
        self.alpha_normalize_recursive(0)
    }

    /// Applies fixed point reduction, which transforms an expression
    /// of the form 𝐘x:A.B into (λx:A.B)(𝐘x:A.B).
    ///
    /// If such a reduction is possible, this method will succeed
    /// as many times as it is called. To prevent such behavior,
    /// `.normalize()` should be called before each reduction.
    ///
    /// It can optionally require that B is a lambda expression, to
    /// avoid expanding recursively defined inductive types.
    ///
    pub fn fixed_point_reduce(self: &mut Self, include_inductive: bool) -> bool {
        let self_clone = self.clone();
        if let Self::FixedPoint {
            binding_identifier,
            binding_type,
            value_term,
            debug_context,
        } = self
        {
            let perform = if let Self::Lambda {
                binding_identifier: _,
                binding_type: _,
                value_term: _,
                debug_context: _,
            } = **value_term
            {
                true
            } else {
                false
            };
            if perform || include_inductive {
                *self = Self::Application {
                    function_term: Box::new(Self::Lambda {
                        binding_identifier: binding_identifier.clone(),
                        binding_type: Box::new(*binding_type.clone()),
                        value_term: Box::new(*value_term.clone()),
                        debug_context: debug_context.clone(),
                    }),
                    parameter_term: Box::new(self_clone),
                    debug_context: debug_context.clone(),
                };
                return true;
            }
        }
        match self {
            Self::Identifier(_, _) => false,
            Self::Application {
                function_term,
                parameter_term,
                debug_context: _,
            } => {
                function_term.fixed_point_reduce(include_inductive)
                    || parameter_term.fixed_point_reduce(include_inductive)
            }
            Self::Lambda {
                binding_identifier: _,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::Forall {
                binding_identifier: _,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::FixedPoint {
                binding_identifier: _,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                value_term.fixed_point_reduce(include_inductive)
                    || binding_type.fixed_point_reduce(include_inductive)
            }
            Self::Constructor {
                type_term,
                value_term,
                debug_context: _,
            } => {
                value_term.fixed_point_reduce(include_inductive)
                    || type_term.fixed_point_reduce(include_inductive)
            }
        }
    }

    /// Applies `.delta_normalize()`, followed by alternating
    /// `.normalize()` and `.fixed_point_reduce()`, and finally
    /// `.alpha_normalize()`.
    ///
    /// Two equivalent terms always become identical after applying
    /// this method on both.
    ///
    /// Please note that no type checking is performed.
    ///
    pub fn full_normalize(self: &mut Self, state: &State) {
        self.delta_normalize(state);
        loop {
            self.normalize();
            if !self.fixed_point_reduce(false) {
                break;
            }
        }
        self.alpha_normalize();
    }

    /// Applies `.delta_normalize()`, followed by alternating
    /// `.normalize()` and `.fixed_point_reduce()`.
    ///
    /// Please note that no type checking is performed.
    ///
    pub fn homonymous_normalize(self: &mut Self, state: &State) {
        self.delta_normalize(state);
        loop {
            self.normalize();
            if !self.fixed_point_reduce(false) {
                break;
            }
        }
    }

    fn get_debug_context<'a>(self: &'a Self) -> &'a TermDebugContext {
        match self {
            Self::Identifier(_, db) => db,
            Self::Application {
                function_term: _,
                parameter_term: _,
                debug_context,
            } => debug_context,
            Self::Lambda {
                binding_identifier: _,
                binding_type: _,
                value_term: _,
                debug_context,
            }
            | Self::Forall {
                binding_identifier: _,
                binding_type: _,
                value_term: _,
                debug_context,
            }
            | Self::FixedPoint {
                binding_identifier: _,
                binding_type: _,
                value_term: _,
                debug_context,
            } => debug_context,
            Self::Constructor {
                type_term: _,
                value_term: _,
                debug_context,
            } => debug_context,
        }
    }

    /// Collect an expression of the form A B C D ...
    /// into (A, [B, C, D, ...])
    fn collect_application(self: &Self) -> (&Self, Vec<&Self>) {
        match self {
            Self::Application {
                function_term,
                parameter_term,
                debug_context: _,
            } => {
                let mut remaining_params = function_term.collect_application();
                remaining_params.1.push(parameter_term);
                remaining_params
            }
            _ => (self, vec![]),
        }
    }

    /// Collect an expression of the form λx:A.λy:B.λz:C. ... Z
    /// into (Z, [..., (z, C), (y, B), (x, A)])
    fn collect_lambda(self: &Self) -> (&Self, Vec<(&String, &Self)>) {
        match self {
            Self::Lambda {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                let mut remaining_params = value_term.collect_lambda();
                remaining_params.1.push((binding_identifier, binding_type));
                remaining_params
            }
            _ => (self, vec![]),
        }
    }

    /// Collect an expression of the form ∀x:A.∀y:B.∀z:C. ... Z
    /// into (Z, [..., (z, C), (y, B), (x, A)])
    fn collect_forall(self: &Self) -> (&Self, Vec<(&String, &Self)>) {
        match self {
            Self::Forall {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                let mut remaining_params = value_term.collect_forall();
                remaining_params.1.push((binding_identifier, binding_type));
                remaining_params
            }
            _ => (self, vec![]),
        }
    }

    /// Collect an expression of the form ∀x:A.∀y:B.∀z:C. ... Z
    /// into (Z, [..., (z, C), (y, B), (x, A)])
    fn collect_forall_mut(self: &mut Self) -> (&mut Self, Vec<(&mut String, &mut Self)>) {
        match self {
            Self::Forall {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                let mut remaining_params = value_term.collect_forall_mut();
                remaining_params.1.push((binding_identifier, binding_type));
                remaining_params
            }
            _ => (self, vec![]),
        }
    }

    fn check_strict_positivity(self: &Self, identifier: &String) -> Result<(), KernelError> {
        // parse 'self' as ∀x:A.∀y:B.∀z:C. ... Z
        let (forall_result, forall_params) = self.collect_forall();
        // Z must either be 'identifier' or not contain it at all
        if forall_result != &Self::Identifier(identifier.clone(), TermDebugContext::Ignore)
            && forall_result.contains(identifier)
        {
            return Err(KernelError::MisshapenInductiveDefinition {
                unexpected_subterm: forall_result.clone(),
                subterm_context: forall_result.get_debug_context().clone(),
                full_term_context: self.get_debug_context().clone(),
            });
        }
        for param in forall_params {
            // parse each parameter type as ∀x:A.∀y:B.∀z:C. ... Z
            let (param_result, param_params) = param.1.collect_forall();
            // Z must either be 'identifier' or not contain it at all
            if param_result != &Self::Identifier(identifier.clone(), TermDebugContext::Ignore)
                && param_result.contains(identifier)
            {
                return Err(KernelError::MisshapenInductiveDefinition {
                    unexpected_subterm: param_result.clone(),
                    subterm_context: param_result.get_debug_context().clone(),
                    full_term_context: self.get_debug_context().clone(),
                });
            }
            for param_param in param_params {
                // parameter types must not contain 'identifier' at all
                if param_param.1.contains(identifier) {
                    return Err(KernelError::NegativeInductiveDefinition {
                        negative_subterm: param_param.1.clone(),
                        subterm_context: param_param.1.get_debug_context().clone(),
                        full_term_context: self.get_debug_context().clone(),
                    });
                }
            }
        }
        Ok(())
    }

    fn check_strictly_decreasing_helper(
        self: &Self,
        index: usize,
        parameter: &String,
        allowed_parameters: &Vec<&String>,
        recursive: &String,
    ) -> bool {
        match self {
            Self::Identifier(s, _) => s != recursive,
            Self::Application {
                function_term: _,
                parameter_term: _,
                debug_context: _,
            } => {
                // parse 'self' as A B C D ...
                let (application_function, application_params) = self.collect_application();
                if application_function
                    == &Self::Identifier(recursive.clone(), TermDebugContext::Ignore)
                {
                    // if A equals 'recursive'
                    for allowed_parameter in allowed_parameters {
                        // the 'index'-th parameter is in the list
                        if application_params[index]
                            == &Self::Identifier(
                                (*allowed_parameter).clone(),
                                TermDebugContext::Ignore,
                            )
                        {
                            return true;
                        }
                    }
                    false
                } else {
                    for application_param in application_params {
                        if !application_param.check_strictly_decreasing_helper(
                            index,
                            parameter,
                            allowed_parameters,
                            recursive,
                        ) {
                            return false;
                        }
                    }
                    application_function.check_strictly_decreasing_helper(
                        index,
                        parameter,
                        allowed_parameters,
                        recursive,
                    )
                }
            }
            Self::Lambda {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::Forall {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::FixedPoint {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                binding_type.check_strictly_decreasing_helper(
                    index,
                    parameter,
                    allowed_parameters,
                    recursive,
                ) && if binding_identifier == parameter {
                    value_term.check_strictly_decreasing(index, &"".to_string(), recursive)
                } else if binding_identifier == recursive {
                    true
                } else {
                    for n in 0..allowed_parameters.len() {
                        if binding_identifier == allowed_parameters[n] {
                            let mut new_allowed_parameters = allowed_parameters.clone();
                            new_allowed_parameters.remove(n);
                            return value_term.check_strictly_decreasing_helper(
                                index,
                                &"".to_string(),
                                &new_allowed_parameters,
                                recursive,
                            );
                        }
                    }
                    value_term.check_strictly_decreasing_helper(
                        index,
                        &"".to_string(),
                        allowed_parameters,
                        recursive,
                    )
                }
            }
            Self::Constructor {
                type_term,
                value_term,
                debug_context: _,
            } => {
                type_term.check_strictly_decreasing_helper(
                    index,
                    parameter,
                    allowed_parameters,
                    recursive,
                ) && value_term.check_strictly_decreasing_helper(
                    index,
                    &"".to_string(),
                    allowed_parameters,
                    recursive,
                )
            }
        }
    }

    fn check_strictly_decreasing(
        self: &Self,
        index: usize,
        parameter: &String,
        recursive: &String,
    ) -> bool {
        match self {
            Self::Identifier(s, _) => s != recursive,
            Self::Application {
                function_term: _,
                parameter_term: _,
                debug_context: _,
            } => {
                // parse 'self' as A B C D ...
                let (application_function, application_params) = self.collect_application();
                if !application_function.check_strictly_decreasing(index, parameter, recursive) {
                    return false;
                }
                let match_entered = application_function
                    == &Self::Identifier(parameter.clone(), TermDebugContext::Ignore);
                for application_param in application_params {
                    if match_entered {
                        // if A is 'parameter', then
                        // parse each sub-parameter as λx:A.λy:B.λz:C. ... Z
                        let (lambda_result, lambda_params) = application_param.collect_lambda();
                        // check A, B, C ...
                        for lambda_param in &lambda_params {
                            if !lambda_param
                                .1
                                .check_strictly_decreasing(index, parameter, recursive)
                            {
                                return false;
                            }
                        }
                        // check Z
                        return lambda_result.check_strictly_decreasing_helper(
                            index,
                            parameter,
                            &lambda_params.into_iter().map(|x| x.0).collect(),
                            recursive,
                        );
                    } else {
                        if !application_param.check_strictly_decreasing(index, parameter, recursive)
                        {
                            return false;
                        }
                    }
                }
                true
            }
            Self::Lambda {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::Forall {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            }
            | Self::FixedPoint {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                binding_type.check_strictly_decreasing(index, parameter, recursive)
                    && if binding_identifier == parameter {
                        value_term.check_strictly_decreasing(index, &"".to_string(), recursive)
                    } else if binding_identifier == recursive {
                        true
                    } else {
                        value_term.check_strictly_decreasing(index, parameter, recursive)
                    }
            }
            Self::Constructor {
                type_term,
                value_term,
                debug_context: _,
            } => {
                type_term.check_strictly_decreasing(index, parameter, recursive)
                    && value_term.check_strictly_decreasing(index, parameter, recursive)
            }
        }
    }

    /// Build an expression of the form A B C D ...
    /// from tuple (A, [B, C, D, ...])
    fn build_application(input: (&Self, &[Self])) -> Self {
        if input.1.len() >= 1 {
            Self::Application {
                function_term: Box::new(Self::build_application((
                    input.0,
                    &input.1[0..input.1.len() - 1],
                ))),
                parameter_term: Box::new(input.1[input.1.len() - 1].clone()),
                debug_context: TermDebugContext::Ignore,
            }
        } else {
            input.0.clone()
        }
    }

    /// Build an expression of the form λx:A.λy:B.λz:C. ... Z
    /// from tuple (Z, [..., (z, C), (y, B), (x, A)])
    fn build_lambda(input: (&Self, &[(&String, &Self)])) -> Self {
        if input.1.len() >= 1 {
            Self::Lambda {
                binding_identifier: input.1[input.1.len() - 1].0.clone(),
                binding_type: Box::new(input.1[input.1.len() - 1].1.clone()),
                value_term: Box::new(Self::build_lambda((
                    input.0,
                    &input.1[0..input.1.len() - 1],
                ))),
                debug_context: TermDebugContext::Ignore,
            }
        } else {
            input.0.clone()
        }
    }

    fn match_replace(
        self: &mut Self,
        identifier: &String,
        generic_identifier: &String,
        parameter_term: &Self,
        inductive_type: &Self,
    ) {
        // create a template for new instances
        let mut instance_template = self.clone();
        let (template_result, mut template_params) = instance_template.collect_forall_mut();
        let constructor_count = template_params.len() - 1;
        let arbitrary_identifier = template_params[constructor_count].0.clone();
        let template_constructors = &mut template_params[..constructor_count];
        // rename parameters to avoid name collisions
        for n in 0..constructor_count {
            let mut suffix: u64 = 0;
            let mut new_identifier;
            'rep: loop {
                new_identifier = format!("{}{}", template_constructors[n].0, suffix);
                for m in 0..n {
                    if &new_identifier == template_constructors[m].0
                        || template_constructors[m].1.contains(&new_identifier)
                    {
                        suffix += 1;
                        continue 'rep;
                    }
                }
                if template_result.contains(&new_identifier) {
                    suffix += 1;
                    continue 'rep;
                }
                break;
            }
            for m in 0..n {
                template_constructors[m].1.replace(
                    template_constructors[n].0,
                    &Self::Identifier(new_identifier.clone(), TermDebugContext::Ignore),
                );
            }
            template_result.replace(
                template_constructors[n].0,
                &Self::Identifier(new_identifier.clone(), TermDebugContext::Ignore),
            );
            *template_constructors[n].0 = new_identifier;
        }
        // now generate a list of instances with variables bound to the template constructor
        let mut instances = vec![];
        for n in 0..constructor_count {
            let template_constructor = &mut template_constructors[n].1;
            let (template_constructor_result, mut template_constructor_params) =
                template_constructor.collect_forall_mut();
            // again, rename parameters to avoid name collisions
            for c in 0..template_constructor_params.len() {
                let mut suffix: u64 = 0;
                let mut new_identifier;
                'rep: loop {
                    new_identifier = format!("{}{}", template_constructor_params[c].0, suffix);
                    for m in 0..c {
                        if &new_identifier == template_constructor_params[m].0
                            || template_constructor_params[m].1.contains(&new_identifier)
                        {
                            suffix += 1;
                            continue 'rep;
                        }
                    }
                    for m in 0..constructor_count {
                        if &new_identifier == template_constructors[m].0 {
                            suffix += 1;
                            continue 'rep;
                        }
                    }
                    if template_constructor_result.contains(&new_identifier) {
                        suffix += 1;
                        continue 'rep;
                    }
                    break;
                }
                let old_identifier = template_constructor_params[c].0.clone();
                for m in 0..c {
                    template_constructor_params[m].1.replace(
                        &old_identifier,
                        &Self::Identifier(new_identifier.clone(), TermDebugContext::Ignore),
                    );
                }
                template_constructor_result.replace(
                    template_constructor_params[c].0,
                    &Self::Identifier(new_identifier.clone(), TermDebugContext::Ignore),
                );
                *template_constructor_params[c].0 = new_identifier;
            }
            // build up inductive instance
            let application = Self::build_application((
                &Self::Identifier(template_constructors[n].0.clone(), TermDebugContext::Ignore),
                &template_constructor_params
                    .iter()
                    .map(|x| Self::Identifier(x.0.clone(), TermDebugContext::Ignore))
                    .rev()
                    .collect::<Vec<_>>(),
            ));
            let tmp = template_constructors
                .iter()
                .map(|x| (&*x.0, &*x.1))
                .collect::<Vec<(&String, &Term)>>();
            let instance = Self::Constructor {
                type_term: Box::new(inductive_type.clone()),
                value_term: Box::new(Self::Lambda {
                    binding_identifier: generic_identifier.clone(),
                    binding_type: Box::new(Self::Identifier(
                        "Type".to_string(),
                        TermDebugContext::Ignore,
                    )),
                    value_term: Box::new(Self::Lambda {
                        binding_identifier: arbitrary_identifier.clone(),
                        binding_type: Box::new(Self::Identifier(
                            generic_identifier.clone(),
                            TermDebugContext::Ignore,
                        )),
                        value_term: Box::new(Self::build_lambda((&application, &*tmp))),
                        debug_context: TermDebugContext::Ignore,
                    }),
                    debug_context: TermDebugContext::Ignore,
                }),
                debug_context: TermDebugContext::Ignore,
            };
            instances.push(instance);
        }
        // then operate on the actual constructors
        let (self_result, mut self_params) = self.collect_forall_mut();
        let constructors = &mut self_params[..constructor_count];
        for n in 0..constructor_count {
            let mut replaced_parameter = parameter_term.clone();
            replaced_parameter.replace(identifier, &instances[n]);
            let mut modified_constructor = template_constructors[n].1.clone();
            let (result, _) = modified_constructor.collect_forall_mut();
            result.replace(&arbitrary_identifier, &replaced_parameter);
            *constructors[n].1 = modified_constructor;
        }
        // handle the result
        self_result.replace(&arbitrary_identifier, parameter_term);

        if let Self::Forall {
            binding_identifier: _,
            binding_type: _,
            value_term,
            debug_context: _,
        } = self
        {
            *self = *value_term.clone();
        }
    }

    fn is_captured(identifier: &String, state: &State, stack: &mut Vec<(String, Self)>) -> bool {
        for (identifier2, _) in stack {
            if identifier == identifier2 {
                return true;
            }
        }
        for (identifier2, _) in &state.terms {
            if identifier == identifier2 {
                return true;
            }
        }
        return false;
    }

    fn infer_type_recursive(
        self: &Self,
        state: &State,
        stack: &mut Vec<(String, Self)>,
    ) -> Result<Self, KernelError> {
        match self {
            Self::Identifier(s, db) => {
                // Set and Prop are type Type
                if s == "Set" || s == "Prop" {
                    return Ok(Self::Identifier(
                        "Type".to_string(),
                        TermDebugContext::TypeOf(Box::new(db.clone())),
                    ));
                }
                // Type is type Type0
                if s == "Type" {
                    return Ok(Self::Identifier(
                        "Type0".to_string(),
                        TermDebugContext::TypeOf(Box::new(db.clone())),
                    ));
                }
                // Type{n} is type Type{n+1}
                if s.len() > 4 && &s[..4] == "Type" {
                    match s[4..].parse::<u64>() {
                        Ok(n) => {
                            return Ok(Self::Identifier(
                                format!("Type{}", n + 1),
                                TermDebugContext::TypeOf(Box::new(db.clone())),
                            ));
                        }
                        Err(_) => (),
                    }
                }
                // captured variables get their corresponding type
                for n in (0..stack.len()).rev() {
                    if s == &stack[n].0 {
                        return Ok(stack[n].1.clone());
                    }
                }
                // state variables get their corresponding type
                match state.terms.get(s) {
                    Some(t) => Ok(t.0.clone()),
                    None => Err(KernelError::UndefinedIdentifier {
                        identifier: s.clone(),
                        identifier_context: db.clone(),
                    }),
                }
            }
            Self::Application {
                function_term,
                parameter_term,
                debug_context,
            } => {
                let new_debug_context = TermDebugContext::TypeOf(Box::new(debug_context.clone()));
                let mut normalized_function_term = function_term.clone();
                normalized_function_term.infer_type_recursive(state, stack)?;
                normalized_function_term.homonymous_normalize(state);
                let original_function_type =
                    normalized_function_term.infer_type_recursive(state, stack)?;
                let mut function_type = original_function_type.clone();
                function_type.infer_type_recursive(state, stack)?;
                function_type.homonymous_normalize(state);
                if let Self::FixedPoint {
                    binding_identifier,
                    binding_type: _,
                    value_term,
                    debug_context: _,
                } = &function_type
                {
                    let mut new_function_type = *value_term.clone();
                    new_function_type.replace(&binding_identifier, &function_type);
                    function_type = new_function_type;
                }
                let mut function_type_clone = function_type.clone();
                let mut inductive_type =
                    Self::Identifier("Type".to_string(), TermDebugContext::Ignore);
                match function_type {
                    Self::Forall {
                        binding_identifier,
                        binding_type,
                        value_term,
                        debug_context: _,
                    } => {
                        let mut parameter_type =
                            parameter_term.infer_type_recursive(state, stack)?;
                        parameter_type.infer_type_recursive(state, stack)?;
                        parameter_type.full_normalize(state);
                        let mut expected_parameter_type = binding_type.clone();
                        expected_parameter_type.infer_type_recursive(state, stack)?;
                        expected_parameter_type.full_normalize(state);
                        let mut actual_function_term = normalized_function_term.clone();
                        let mut generic_identifier = "".to_string();
                        let is_match_term = if let Self::Application {
                            function_term: function_term2,
                            parameter_term: _,
                            debug_context: _,
                        } = *normalized_function_term
                        {
                            if let Self::Application {
                                function_term: _,
                                parameter_term: _,
                                debug_context: _,
                            } = *function_term2
                            {
                                false
                            } else {
                                actual_function_term = function_term2.clone();
                                let mut function_term2_type =
                                    function_term2.infer_type_recursive(state, stack)?;
                                inductive_type = function_term2_type.clone();
                                if let Self::FixedPoint {
                                    binding_identifier: _,
                                    binding_type: binding_type2,
                                    value_term: value_term2,
                                    debug_context: _,
                                } = &inductive_type
                                {
                                    if let Self::Identifier(_, _) = &**binding_type2 {
                                        function_term2_type = *value_term2.clone();
                                    }
                                }

                                if let Self::Forall {
                                    binding_identifier: binding_identifier2,
                                    binding_type: binding_type3,
                                    value_term: value_term3,
                                    debug_context: _,
                                } = function_term2_type
                                {
                                    if let Self::Identifier(s2, _) = &*binding_type3 {
                                        if s2 == "Type" {
                                            if let Self::Forall {
                                                binding_identifier: _,
                                                binding_type: binding_type4,
                                                value_term: _,
                                                debug_context: _,
                                            } = &*value_term3
                                            {
                                                if let Self::Identifier(s3, _) = &**binding_type4 {
                                                    generic_identifier =
                                                        binding_identifier2.to_string();
                                                    s3 == &binding_identifier2
                                                } else {
                                                    false
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            }
                        } else {
                            false
                        };
                        if parameter_type != *expected_parameter_type {
                            if !is_match_term {
                                return Err(KernelError::NonmatchingArgument {
                                    expected_type: *expected_parameter_type,
                                    observed_type: parameter_type,
                                    function_context: function_term.get_debug_context().clone(),
                                    parameter_context: parameter_term.get_debug_context().clone(),
                                });
                            }
                        }
                        if is_match_term {
                            if let Self::Identifier(s, _) = &*actual_function_term {
                                function_type_clone.match_replace(
                                    s,
                                    &generic_identifier,
                                    parameter_term,
                                    &inductive_type,
                                );
                                return Ok(function_type_clone);
                            }
                        }
                        let mut output_type = value_term.with_new_debug_context(&new_debug_context);
                        // replace after assigning new debug context,
                        // so that the type of (λx:T.x) P
                        // is shown simply as the type of P
                        output_type.replace(&binding_identifier, parameter_term);
                        Ok(output_type)
                    }
                    _ => Err(KernelError::InvalidApplication {
                        nonfunction_type: function_type,
                        nonfunction_context: function_term.get_debug_context().clone(),
                        parameter_context: parameter_term.get_debug_context().clone(),
                    }),
                }
            }
            Self::Lambda {
                binding_identifier,
                binding_type,
                value_term,
                debug_context,
            } => {
                let mut binding_type_type = binding_type.infer_type_recursive(state, stack)?;
                binding_type_type.infer_type_recursive(state, stack)?;
                binding_type_type.homonymous_normalize(state);
                let valid = if let Self::Identifier(_, _) = &binding_type_type {
                    let mut binding_type_type_type =
                        binding_type_type.infer_type_recursive(state, stack)?;
                    binding_type_type_type.infer_type_recursive(state, stack)?;
                    binding_type_type_type.homonymous_normalize(state);
                    if let Self::Identifier(s, _) = &binding_type_type_type {
                        match &**s {
                            "Type" => true,
                            _ => {
                                s.len() > 4
                                    && &s[..4] == "Type"
                                    && match s[4..].parse::<u64>() {
                                        Ok(_) => true,
                                        Err(_) => false,
                                    }
                            }
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };
                if !valid {
                    return Err(KernelError::InvalidType {
                        incorrect_term: *binding_type.clone(),
                        incorrect_type: binding_type_type.clone(),
                        incorrect_context: binding_type.get_debug_context().clone(),
                    });
                }
                let mut new_binding_identifier = binding_identifier.clone();
                let mut new_value_term = value_term.clone();
                // rename binding identifier to avoid name collisions
                if Self::is_captured(&new_binding_identifier, state, stack) {
                    let mut suffix: u64 = 0;
                    let mut new_identifier;
                    'rep: loop {
                        new_identifier = format!("{}{}", new_binding_identifier, suffix);
                        if Self::is_captured(&new_identifier, state, stack) {
                            suffix += 1;
                            continue 'rep;
                        }
                        break;
                    }
                    new_binding_identifier = new_identifier;
                    new_value_term.replace(
                        binding_identifier,
                        &Self::Identifier(new_binding_identifier.clone(), TermDebugContext::Ignore),
                    );
                }
                stack.push((new_binding_identifier.clone(), *binding_type.clone()));
                let inner_type = new_value_term.infer_type_recursive(state, stack)?;
                stack.pop();
                let output_type = Self::Forall {
                    binding_identifier: new_binding_identifier,
                    binding_type: binding_type.clone(),
                    value_term: Box::new(inner_type),
                    debug_context: TermDebugContext::TypeOf(Box::new(debug_context.clone())),
                };
                Ok(output_type)
            }
            Self::Forall {
                binding_identifier,
                binding_type,
                value_term,
                debug_context,
            } => {
                let mut binding_type_type = binding_type.infer_type_recursive(state, stack)?;
                binding_type_type.infer_type_recursive(state, stack)?;
                binding_type_type.homonymous_normalize(state);
                let valid = if let Self::Identifier(_, _) = &binding_type_type {
                    let mut binding_type_type_type =
                        binding_type_type.infer_type_recursive(state, stack)?;
                    binding_type_type_type.infer_type_recursive(state, stack)?;
                    binding_type_type_type.homonymous_normalize(state);
                    if let Self::Identifier(s, _) = &binding_type_type_type {
                        match &**s {
                            "Type" => true,
                            _ => {
                                s.len() > 4
                                    && &s[..4] == "Type"
                                    && match s[4..].parse::<u64>() {
                                        Ok(_) => true,
                                        Err(_) => false,
                                    }
                            }
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };
                if !valid {
                    return Err(KernelError::InvalidType {
                        incorrect_term: *binding_type.clone(),
                        incorrect_type: binding_type_type.clone(),
                        incorrect_context: binding_type.get_debug_context().clone(),
                    });
                }
                let mut new_binding_identifier = binding_identifier.clone();
                let mut new_value_term = value_term.clone();
                // rename binding identifier to avoid name collisions
                if Self::is_captured(&new_binding_identifier, state, stack) {
                    let mut suffix: u64 = 0;
                    let mut new_identifier;
                    'rep: loop {
                        new_identifier = format!("{}{}", new_binding_identifier, suffix);
                        if Self::is_captured(&new_identifier, state, stack) {
                            suffix += 1;
                            continue 'rep;
                        }
                        break;
                    }
                    new_binding_identifier = new_identifier;
                    new_value_term.replace(
                        binding_identifier,
                        &Self::Identifier(new_binding_identifier.clone(), TermDebugContext::Ignore),
                    );
                }
                stack.push((new_binding_identifier.clone(), *binding_type.clone()));
                let mut inner_type = new_value_term.infer_type_recursive(state, stack)?;
                // replace after inferring inner type,
                // so that the type of ∀x:P.Q
                // is not just shown as the type of Q
                // unless Q doesn't contain x
                if new_value_term.contains(&new_binding_identifier) {
                    inner_type = inner_type.with_new_debug_context(&TermDebugContext::TypeOf(
                        Box::new(debug_context.clone()),
                    ));
                }
                inner_type.infer_type_recursive(state, stack)?;
                inner_type.homonymous_normalize(state);
                let valid = if let Self::Identifier(_, _) = inner_type {
                    let mut inner_type_type = inner_type.infer_type_recursive(state, stack)?;
                    inner_type_type.infer_type_recursive(state, stack)?;
                    inner_type_type.full_normalize(state);
                    if let Self::Identifier(s, _) = &inner_type_type {
                        match &**s {
                            "Type" => true,
                            _ => {
                                s.len() > 4
                                    && &s[..4] == "Type"
                                    && match s[4..].parse::<u64>() {
                                        Ok(_) => true,
                                        Err(_) => false,
                                    }
                            }
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };
                stack.pop();
                if !valid {
                    return Err(KernelError::InvalidType {
                        incorrect_term: *new_value_term.clone(),
                        incorrect_type: inner_type.clone(),
                        incorrect_context: new_value_term.get_debug_context().clone(),
                    });
                }
                if let Self::Identifier(s, _) = &inner_type {
                    if s == &new_binding_identifier {
                        return Ok(Self::Identifier(
                            "Set".to_string(),
                            TermDebugContext::TypeOf(Box::new(self.get_debug_context().clone())),
                        ));
                    }
                }
                Ok(inner_type)
            }
            Self::FixedPoint {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                binding_type.infer_type_recursive(state, stack)?;
                let mut new_binding_identifier = binding_identifier.clone();
                let mut new_value_term = value_term.clone();
                // rename binding identifier to avoid name collisions
                if Self::is_captured(&new_binding_identifier, state, stack) {
                    let mut suffix: u64 = 0;
                    let mut new_identifier;
                    'rep: loop {
                        new_identifier = format!("{}{}", new_binding_identifier, suffix);
                        if Self::is_captured(&new_identifier, state, stack) {
                            suffix += 1;
                            continue 'rep;
                        }
                        break;
                    }
                    new_binding_identifier = new_identifier;
                    new_value_term.replace(
                        binding_identifier,
                        &Self::Identifier(new_binding_identifier.clone(), TermDebugContext::Ignore),
                    );
                }
                stack.push((new_binding_identifier.clone(), *binding_type.clone()));
                let inner_type = new_value_term.infer_type_recursive(state, stack)?;
                if inner_type.contains(&new_binding_identifier) {
                    return Err(KernelError::SelfReferencingRecursiveType {
                        full_term_context: self.get_debug_context().clone(),
                    });
                }
                new_value_term.homonymous_normalize(state);
                match *new_value_term {
                    Self::Lambda {
                        binding_identifier: _,
                        binding_type: _,
                        value_term: _,
                        debug_context: _,
                    } => {
                        // λx1:A1.λx2:A2. ... C
                        // at least one Ai is of type Set
                        // and its xi is applied on a closure
                        // λz1:B1.λz2:B2. ... λyi:Ai.D
                        // such that yi is the ith argument
                        // to any binding_identifier within D
                        let mut current_term = new_value_term;
                        let mut parameter_list = vec![];
                        loop {
                            if let Self::Lambda {
                                binding_identifier: current_binding_identifier,
                                binding_type: current_binding_type,
                                value_term: current_value_term,
                                debug_context: _,
                            } = *current_term
                            {
                                parameter_list.push((
                                    current_binding_identifier.clone(),
                                    current_binding_type.clone(),
                                ));
                                current_term = current_value_term;
                            } else {
                                break;
                            }
                        }
                        let mut valid = false;
                        for n in 0..parameter_list.len() {
                            let parameter_type = &parameter_list[n].1;
                            if parameter_type.contains(&new_binding_identifier) {
                                break;
                            }
                            let parameter_type_type =
                                parameter_type.infer_type_recursive(state, stack)?;
                            if let Self::Identifier(s, _) = parameter_type_type {
                                if s == "Set" {
                                    if current_term.check_strictly_decreasing(
                                        n,
                                        &parameter_list[n].0,
                                        &new_binding_identifier,
                                    ) {
                                        for _m in 0..n {
                                            stack.pop();
                                        }
                                        valid = true;
                                        break;
                                    }
                                }
                            }
                            stack.push((parameter_list[n].0.clone(), *parameter_list[n].1.clone()));
                        }
                        if !valid {
                            return Err(KernelError::NonprimitiveRecursiveFunction {
                                full_term_context: self.get_debug_context().clone(),
                            });
                        }
                    }
                    Self::Forall {
                        binding_identifier: _,
                        binding_type: _,
                        value_term: _,
                        debug_context: _,
                    } => {
                        // ∀x1:A1.∀x2:A2. ... C
                        // every Ai can only contain binding_identifier
                        // at strictly positive positions.
                        let mut current_term = new_value_term;
                        loop {
                            if let Self::Forall {
                                binding_identifier: _,
                                binding_type: current_binding_type,
                                value_term: current_value_term,
                                debug_context: _,
                            } = *current_term
                            {
                                current_binding_type
                                    .check_strict_positivity(&new_binding_identifier)?;
                                current_term = current_value_term;
                            } else {
                                break;
                            }
                        }
                    }
                    _ => {
                        return Err(KernelError::MisshapenRecursiveDefinition {
                            unexpected_subterm: *new_value_term.clone(),
                            subterm_context: new_value_term.get_debug_context().clone(),
                            full_term_context: self.get_debug_context().clone(),
                        })
                    }
                }
                stack.pop();
                Ok(inner_type)
            }
            Self::Constructor {
                type_term,
                value_term,
                debug_context: _,
            } => {
                let mut value_term_type = value_term.infer_type_recursive(state, stack)?;
                value_term_type.full_normalize(state);
                let mut expanded_type_term = *type_term.clone();
                expanded_type_term.infer_type_recursive(state, stack)?;
                expanded_type_term.homonymous_normalize(state);
                expanded_type_term.fixed_point_reduce(true);
                expanded_type_term.full_normalize(state);
                if value_term_type == expanded_type_term {
                    return Ok(*type_term.clone());
                } else {
                    return Err(KernelError::InvalidInstance {});
                }
            }
        }
    }

    /// Infers the type of a term, which is in turn also a term.
    ///
    /// Will return an error if the given term is invalid.
    ///
    pub fn infer_type(self: &Self, state: &State) -> Result<Self, KernelError> {
        let mut stack = vec![];
        self.infer_type_recursive(state, &mut stack)
        // Clear stack afterwards
    }
}

impl std::fmt::Debug for Term {
    /// Formats the given term as Calculus of Contructions.
    ///
    /// Uses symbols λ and ∀, plus two abbreviations:
    /// * ∀x:A.B as A→B if B doesn't contain x.
    /// * ∀x:A.x as ⊥.
    ///
    /// Needless to say, this code is not part of the
    /// trusted base.
    ///
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Term::Identifier(s, _) => {
                f.write_str(s)?;
            }
            Term::Application {
                function_term,
                parameter_term,
                debug_context: _,
            } => {
                let parenthesize_first = match **function_term {
                    Term::Lambda { .. } | Term::Forall { .. } | Term::FixedPoint { .. } => true,
                    _ => false,
                };
                if parenthesize_first {
                    f.write_str("(")?;
                }
                function_term.fmt(f)?;
                if parenthesize_first {
                    f.write_str(")")?;
                }
                f.write_str(" ")?;
                let parenthesize_second = match **parameter_term {
                    Term::Application { .. } => true,
                    Term::Lambda { .. } | Term::Forall { .. } | Term::FixedPoint { .. } => true,
                    _ => false,
                };
                if parenthesize_second {
                    f.write_str("(")?;
                }
                parameter_term.fmt(f)?;
                if parenthesize_second {
                    f.write_str(")")?;
                }
            }
            Term::Lambda {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                f.write_str("λ")?;
                f.write_str(binding_identifier)?;
                f.write_str(":")?;
                binding_type.fmt(f)?;
                f.write_str(".")?;
                value_term.fmt(f)?;
            }
            Term::Forall {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                if !value_term.contains(binding_identifier) {
                    // special case: A→B := ∀x:A.B
                    let parenthesize_first = match **binding_type {
                        Term::Lambda { .. } | Term::Forall { .. } => true,
                        _ => false,
                    };
                    if parenthesize_first {
                        f.write_str("(")?;
                    }
                    binding_type.fmt(f)?;
                    if parenthesize_first {
                        f.write_str(")")?;
                    }
                    f.write_str("→")?;
                    value_term.fmt(f)?;
                } else if if let Term::Identifier(s, _) = &**value_term {
                    s == binding_identifier
                        && **binding_type
                            == Term::Identifier("Prop".to_string(), TermDebugContext::Ignore)
                } else {
                    false
                } {
                    // special case: ⊥ := ∀x:A.x
                    f.write_str("⊥")?;
                } else {
                    // ∀x:A.B where B contains x
                    f.write_str("∀")?;
                    f.write_str(binding_identifier)?;
                    f.write_str(":")?;
                    binding_type.fmt(f)?;
                    f.write_str(".")?;
                    value_term.fmt(f)?;
                }
            }
            Term::FixedPoint {
                binding_identifier,
                binding_type,
                value_term,
                debug_context: _,
            } => {
                f.write_str("𝐘")?;
                f.write_str(binding_identifier)?;
                f.write_str(":")?;
                binding_type.fmt(f)?;
                f.write_str(".")?;
                value_term.fmt(f)?;
            }
            Term::Constructor {
                type_term,
                value_term,
                debug_context: _,
            } => {
                f.write_str("℺")?;
                type_term.fmt(f)?;
                f.write_str(".")?;
                value_term.fmt(f)?;
            }
        }
        Result::Ok(())
    }
}

impl State {
    /// Creates a new context containing no terms.
    pub fn new() -> Self {
        State {
            terms: std::collections::HashMap::new(),
        }
    }

    /// Attempts to define a new variable in the given state.
    ///
    /// This is equivalent to `let x: A = B;`.
    /// Will return an error if either term is invalid
    /// or the type of `B` doesn't match `A`.
    ///
    pub fn try_define(
        self: &mut Self,
        identifier: &String,
        type_term: Term,
        value_term: Term,
    ) -> Result<(), KernelError> {
        let mut type_term_normalized = type_term.clone();
        type_term_normalized.delta_normalize(self);
        type_term_normalized.infer_type(&self)?;
        type_term_normalized.normalize();
        let mut value_term_normalized = value_term.clone();
        value_term_normalized.delta_normalize(self);
        let mut actual_type = value_term_normalized.infer_type(&self)?;
        value_term_normalized.normalize();
        actual_type.full_normalize(self);
        let mut expected_type = type_term_normalized.clone();
        expected_type.full_normalize(self);
        if expected_type != actual_type {
            return Err(KernelError::NonmatchingDefinition {
                expected_type: expected_type,
                observed_type: actual_type,
                signature_context: type_term.get_debug_context().clone(),
                definition_context: value_term.get_debug_context().clone(),
            });
        }
        self.terms
            .insert(identifier.clone(), (type_term, value_term));
        Ok(())
    }
}
