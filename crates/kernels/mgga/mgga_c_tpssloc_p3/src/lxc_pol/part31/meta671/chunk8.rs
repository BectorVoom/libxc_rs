//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2009/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2009<F: Float>(t111: F, t28942: F, t5456: F, t7039: F, t100990: F, t102310: F, t1268: F, t12725: F, t1458: F, t19451: F, t19456: F, t2039: F, t27170: F, t28002: F, t4028: F, t671: F, t7056: F, t75560: F, t7801: F, t92090: F, t96356: F, t96683: F, t96709: F) -> (F, F, F) {
    let t102386 = t28942 * t111;
    let t102401 = t7039 * t5456;
    let t102403 = F::cast_from(2.0_f64) * t100990 * t1268 + F::cast_from(2.0_f64) * t102386 * t671 + F::cast_from(4.0_f64) * t12725 * t7801 + F::cast_from(4.0_f64) * t1458 * t92090 + F::cast_from(2.0_f64) * t19451 * t7056 + F::cast_from(4.0_f64) * t19456 * t7801 + F::cast_from(2.0_f64) * t2039 * t75560 + F::cast_from(4.0_f64) * t2039 * t96356 + F::cast_from(4.0_f64) * t2039 * t96683 + F::cast_from(2.0_f64) * t2039 * t96709 + F::cast_from(4.0_f64) * t27170 * t4028 + F::cast_from(4.0_f64) * t28002 * t7056 + t102310 + F::cast_from(2.0_f64) * t102401;
    (t102386, t102401, t102403)
}
