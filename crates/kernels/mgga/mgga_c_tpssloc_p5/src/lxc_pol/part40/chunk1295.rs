//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1295/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1295<F: Float>(t1266: F, t1268: F, t1774: F, t19456: F, t2181: F, t2183: F, t2314: F, t28002: F, t30180: F, t30181: F, t30189: F, t30211: F, t30424: F, t30425: F, t30428: F, t30444: F, t30447: F, t30454: F, t4028: F, t4034: F, t5107: F, t5113: F, t5361: F, t6468: F, t652: F, t8124: F, t8143: F, t8230: F, t8235: F, t96356: F, t96683: F) -> F {
    let t111503 = -F::cast_from(2.0_f64) * t1266 * t30424 * t652 + F::cast_from(4.0_f64) * t1268 * t5361 * t8230 + F::cast_from(2.0_f64) * t1268 * t6468 * t8143 - F::cast_from(4.0_f64) * t1774 * t30180 * t652 - F::cast_from(4.0_f64) * t5107 * t652 * t8230 + F::cast_from(4.0_f64) * t19456 * t8235 - F::cast_from(4.0_f64) * t2181 * t96356 + F::cast_from(4.0_f64) * t2183 * t96683 + F::cast_from(2.0_f64) * t2314 * t30425 + F::cast_from(4.0_f64) * t2314 * t30428 - F::cast_from(4.0_f64) * t2314 * t30444 - F::cast_from(2.0_f64) * t2314 * t30447 - F::cast_from(4.0_f64) * t28002 * t8124 + F::cast_from(4.0_f64) * t30181 * t4028 - F::cast_from(4.0_f64) * t30189 * t4028 + F::cast_from(4.0_f64) * t30211 * t4028 + F::cast_from(2.0_f64) * t30425 * t5113 - F::cast_from(4.0_f64) * t30444 * t4034 - F::cast_from(2.0_f64) * t30447 * t4034 + F::cast_from(2.0_f64) * t30454 * t5113;
    t111503
}
