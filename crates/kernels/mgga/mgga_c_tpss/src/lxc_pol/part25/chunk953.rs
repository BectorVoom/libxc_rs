//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 953/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk953<F: Float>(t1098: F, t12384: F, t1561: F, t3054: F, t1127: F, t2840: F, t11453: F, t4279: F, t1125: F, t4233: F, t3052: F, t1569: F, t2719: F) -> (F, F, F, F, F, F) {
    let t12385 = t1098 * t12384;
    let t12387 = t1561 * t3054;
    let t12399 = t1127 * t2840;
    let t12404 = t11453 * t4279;
    let t12406 = F::new(5.0) / F::new(10368.0) * t1125 * t12404;
    let t12407 = t11453 * t4233;
    let t12409 = t3052 * t12407 / F::new(1152.0);
    let t12429 = t1569 * t2719;
    (t12385, t12387, t12399, t12406, t12409, t12429)
}
