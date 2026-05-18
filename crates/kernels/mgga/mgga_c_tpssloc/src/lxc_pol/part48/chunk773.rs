//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 773/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk773<F: Float>(t24217: F, t24233: F, t218: F, t7084: F, t798: F, t23013: F, t23031: F, t2684: F, t7101: F, t2047: F, t2627: F, t2633: F) -> (F, F, F, F, F, F, F) {
    let t24234 = t24217 + t24233;
    let t24235 = t218 * t24234;
    let t24237 = t798 * t7084;
    let t24246 = F::new(0.12793931631041761173e0) * t23013;
    let t24250 = F::new(0.52089578783527170489e-1) * t23031;
    let t24251 = t7101 * t2684;
    let t24255 = t2627 * t2047;
    let t24256 = t24255 * t2633;
    (t24234, t24235, t24237, t24246, t24250, t24251, t24256)
}
