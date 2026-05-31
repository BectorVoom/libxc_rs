//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1131/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1131<F: Float>(t1268: F, t12725: F, t1458: F, t19456: F, t2039: F, t2314: F, t23938: F, t26114: F, t26117: F, t26967: F, t26977: F, t27170: F, t27188: F, t4028: F, t4072: F, t5113: F, t671: F, t7042: F, t7056: F, t7676: F, t7801: F) -> F {
    let t27215 = F::cast_from(2.0_f64) * t1268 * t27170 + F::cast_from(2.0_f64) * t12725 * t2039 + F::cast_from(2.0_f64) * t1458 * t23938 + F::cast_from(2.0_f64) * t1458 * t26977 + F::cast_from(2.0_f64) * t19456 * t2039 + F::cast_from(2.0_f64) * t2039 * t26114 + F::cast_from(2.0_f64) * t2039 * t26117 + F::cast_from(2.0_f64) * t2314 * t7801 + F::cast_from(2.0_f64) * t27188 * t671 + F::cast_from(2.0_f64) * t4028 * t7056 + F::cast_from(2.0_f64) * t4072 * t7042 + F::cast_from(2.0_f64) * t5113 * t7801 + F::cast_from(2.0_f64) * t7056 * t7676 + t26967;
    t27215
}
