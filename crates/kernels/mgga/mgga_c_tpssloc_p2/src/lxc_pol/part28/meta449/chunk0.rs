//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1640/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1640<F: Float>(t5: F, t24006: F, t112: F, t1268: F, t12734: F, t12739: F, t2039: F, t2314: F, t2363: F, t23917: F, t23938: F, t23941: F, t5113: F, t671: F, t7042: F, t7056: F, t9348: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t24007 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t24006);
    let t24008 = t24007 * t112;
    let t24026 = F::cast_from(2.0_f64) * t1268 * t23917 + F::cast_from(4.0_f64) * t12734 * t2039 + F::cast_from(2.0_f64) * t12739 * t2039 + F::cast_from(2.0_f64) * t2039 * t9348 + F::cast_from(4.0_f64) * t2314 * t7056 + F::cast_from(2.0_f64) * t2363 * t7042 + F::cast_from(4.0_f64) * t23938 * t671 + F::cast_from(4.0_f64) * t5113 * t7056 + F::cast_from(2.0_f64) * t23941 + t24008;
    (t24007, t24008, t24026)
}
