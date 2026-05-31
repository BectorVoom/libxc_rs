//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1964/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1964<F: Float>(t109: F, t86586: F, t86588: F, t86590: F, t81440: F, t81443: F, t81445: F, t84036: F, t86593: F, t86596: F, t86599: F, t86601: F, t1268: F, t12725: F, t12734: F, t12739: F, t19456: F, t2039: F, t2314: F, t23917: F, t26114: F, t26117: F, t27170: F, t5113: F, t55934: F, t7056: F, t7676: F, t7801: F, t90370: F, t90375: F, t9348: F) -> (F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t92121 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t86586;
    let t92122 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t86588;
    let t92123 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t86590;
    let t92127 = -t84036 - F::cast_from(44.0_f64) / F::cast_from(9.0_f64) * t81440 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t81443 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t81445 - t92121 - t92122 + t92123 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t86593 + t86596 + t86599 / F::cast_from(2.0_f64) - t86601 / F::cast_from(4.0_f64);
    let t92128 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t92127);
    let t92139 = F::cast_from(2.0_f64) * t1268 * t92128 + F::cast_from(4.0_f64) * t12725 * t7056 + F::cast_from(4.0_f64) * t12734 * t7801 + F::cast_from(2.0_f64) * t12739 * t7801 + F::cast_from(4.0_f64) * t19456 * t7056 + F::cast_from(4.0_f64) * t2039 * t55934 + F::cast_from(4.0_f64) * t2039 * t90370 + F::cast_from(2.0_f64) * t2039 * t90375 + F::cast_from(4.0_f64) * t2314 * t27170 + F::cast_from(2.0_f64) * t23917 * t7676 + F::cast_from(4.0_f64) * t26114 * t7056 + F::cast_from(4.0_f64) * t26117 * t7056 + F::cast_from(4.0_f64) * t27170 * t5113 + F::cast_from(2.0_f64) * t7801 * t9348;
    (t92128, t92139)
}
