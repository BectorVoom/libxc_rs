//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1122/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1122<F: Float>(t12394: F, t9702: F, t1127: F, t2840: F, t11476: F, t3931: F, t11453: F, t4279: F, t1125: F, t4233: F, t3052: F, t1501: F, t3081: F) -> (F, F, F, F, F) {
    let t12395 = t9702 * t12394;
    let t12399 = t1127 * t2840;
    let t12400 = t12399 * t11476;
    let t12401 = t3931 * t12400;
    let t12404 = t11453 * t4279;
    let t12406 = F::new(5.0) / F::new(10368.0) * t1125 * t12404;
    let t12407 = t11453 * t4233;
    let t12409 = t3052 * t12407 / F::new(1152.0);
    let t12410 = t1501 * t3081;
    (t12395, t12401, t12406, t12409, t12410)
}
