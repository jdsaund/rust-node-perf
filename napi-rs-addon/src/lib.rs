#[macro_use]
extern crate napi_rs as napi;
#[macro_use]
extern crate napi_rs_derive;

use napi::{CallContext, Env, Number, Object, Result, Value};
use std::convert::TryInto;

register_module!(test_module, init);

fn init<'env>(
  env: &'env Env,
  exports: &'env mut Value<'env, Object>,
) -> Result<Option<Value<'env, Object>>> {

  exports.set_named_property("add", env.create_function("add", add)?)?;
  Ok(None)
}

#[js_function(2)] // ------> arguments length, omit for zero
fn add<'env>(ctx: CallContext<'env>) -> Result<Value<'env, Number>> {
    let a: f64 = ctx.get::<Number>(0)?.try_into().unwrap();
    let b: f64 = ctx.get::<Number>(1)?.try_into().unwrap();
    ctx.env.create_double(a + b)
}
