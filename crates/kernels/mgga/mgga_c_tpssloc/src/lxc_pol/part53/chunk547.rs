//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 547/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk547<F: Float>(t240: F, t6619: F, t812: F, t849: F, t1906: F, t6547: F, t214: F, t225: F) -> (F, F, F, F, F) {
    let t6620 = t6619 * t240;
    let t6621 = t812 * t6620;
    let t6622 = t6621 * t849;
    let t6635 = t6547 * t1906;
    let t6637 = t214 * t225;
    (t6620, t6621, t6622, t6635, t6637)
}
