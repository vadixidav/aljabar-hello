# aljabar-hello
Messing around with aljabar to see what abstractions I can make

## GRU

The first test is to create a Gated Recurrent Unit (GRU). I started with a simple GRU neural net layer, which was comprised of:

- Hidden matrix
- Input matrix
- Biases
- Activation function

You need two of these to create a full GRU. One of them is to control the forget/recall gate, and the other is to determine the part of the output which was forgotten. Basically: `new = old * recall + output * (1 - recall)`. The output and recall vectors both come from the same neural network structure, although the original paper used a sigmoid for one and a tanh for the other. In this case I am just going to abstract over the activation function and let the caller decide it.

Upon trying this, I couldn't even get past the first part. It failed when doing a basic test building the GRU `NNet`:

```rust
error: internal compiler error: src/librustc_metadata/decoder.rs:483: entry: id not found: DefIndex(327) in crate aljabar with number 15

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:649:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error
```

Full stack trace below:

```rust
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:649:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
   8: std::panicking::begin_panic
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::entry
  17: rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::get_type
  18: rustc_metadata::cstore_impl::provide_extern::type_of
  19: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  20: rustc::dep_graph::graph::DepGraph::with_task_impl
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  22: rustc::ty::instance::Instance::resolve
  23: <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_const
  24: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  25: rustc::ty::fold::TypeFoldable::fold_with
  26: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  27: <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty
  28: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  29: rustc::ty::fold::TypeFoldable::fold_with
  30: rustc::traits::project::normalize
  31: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
  32: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  33: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  34: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_struct
  35: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  36: rustc_typeck::check::FnCtxt::check_decl_initializer
  37: rustc_typeck::check::FnCtxt::check_decl_local
  38: rustc_typeck::check::FnCtxt::check_stmt
  39: rustc_typeck::check::FnCtxt::check_block_with_expected
  40: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  41: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  42: rustc_typeck::check::check_fn
  43: rustc::ty::context::GlobalCtxt::enter_local
  44: rustc_typeck::check::typeck_tables_of
  45: rustc::ty::query::__query_compute::typeck_tables_of
  46: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  47: rustc::dep_graph::graph::DepGraph::with_task_impl
  48: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  49: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  50: rustc_typeck::check::typeck_item_bodies
  51: rustc::ty::query::__query_compute::typeck_item_bodies
  52: rustc::dep_graph::graph::DepGraph::with_task_impl
  53: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  54: rustc::util::common::time
  55: rustc_typeck::check_crate
  56: rustc_interface::passes::analysis
  57: rustc::ty::query::__query_compute::analysis
  58: rustc::dep_graph::graph::DepGraph::with_task_impl
  59: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  60: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  61: rustc_interface::passes::create_global_ctxt::{{closure}}
  62: rustc_interface::interface::run_compiler_in_existing_thread_pool
  63: std::thread::local::LocalKey<T>::with
  64: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [type_of] processing `aljabar::Mat2x2::{{constant}}#0`
#1 [typeck_tables_of] processing `gru::test`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
end of query stack
```

## BitArray

I tried to make a simple BitArray. This also failed almost right away:

```rust
error[E0277]: the trait bound `[u8; 1]: std::convert::From<aljabar::Vector<u8, {(B + 7) / 8}>>` is not satisfied
  --> src/bitarray.rs:15:26
   |
15 |     let b: [u8; 1] = b.0.into();
   |                          ^^^^ the trait `std::convert::From<aljabar::Vector<u8, {(B + 7) / 8}>>` is not implemented for `[u8; 1]`
   |
   = note: required because of the requirements on the impl of `std::convert::Into<[u8; 1]>` for `aljabar::Vector<u8, {(B + 7) / 8}>`
```