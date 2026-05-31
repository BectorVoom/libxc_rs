//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 710/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk710<F: Float>(t138: F, t2409: F, t125: F, t2412: F, t701: F, t2414: F) -> (F, F) {
    let t9452 = F::cast_from(1.0_f64) / t2409 / t138;
    let t9453 = t125 * t9452;
    let t9454 = t2412 * t701;
    let t9455 = t9454 * t2414;
    let t9457 = F::cast_from(0.96491876992155210402e2_f64) * t9453 * t9455;
    (t9454, t9457)
}
