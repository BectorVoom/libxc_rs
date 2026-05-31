//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1330/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1330<F: Float>(t1268: F, t12734: F, t12739: F, t12823: F, t1393: F, t19456: F, t2181: F, t2314: F, t26114: F, t29935: F, t29944: F, t30180: F, t30181: F, t30195: F, t30209: F, t30211: F, t3652: F, t3929: F, t4034: F, t45632: F, t5113: F, t652: F, t7458: F, t7676: F, t8124: F, t8150: F, t8221: F, t8230: F, t8231: F, t8235: F, t8237: F, t90370: F, t9348: F) -> F {
    let t110870 = -F::cast_from(2.0_f64) * t652 * t3652 * t8230 + F::cast_from(2.0_f64) * t1268 * t8230 * t3929 + F::cast_from(2.0_f64) * t7676 * t29944 + F::cast_from(4.0_f64) * t19456 * t8150 - F::cast_from(2.0_f64) * t45632 * t2181 - F::cast_from(4.0_f64) * t12734 * t8221 - F::cast_from(4.0_f64) * t2314 * t30209 + F::cast_from(4.0_f64) * t2314 * t30211 + F::cast_from(2.0_f64) * t12739 * t8235 + F::cast_from(4.0_f64) * t5113 * t30181 + F::cast_from(4.0_f64) * t1268 * t30180 * t1393 - F::cast_from(2.0_f64) * t12823 * t8231 - F::cast_from(4.0_f64) * t4034 * t30195 - F::cast_from(2.0_f64) * t7458 * t29935 - F::cast_from(2.0_f64) * t9348 * t8221 - F::cast_from(4.0_f64) * t90370 * t2181 - F::cast_from(4.0_f64) * t26114 * t8124 - F::cast_from(4.0_f64) * t12734 * t8231 - F::cast_from(4.0_f64) * t2314 * t30195 + F::cast_from(2.0_f64) * t12739 * t8237;
    t110870
}
