//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1767/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1767<F: Float>(t1049: F, t362: F, t225: F, t23592: F, t23384: F, t6787: F, t3216: F, t6818: F, t11094: F, t1958: F, t2752: F, t28: F) -> (F, F, F, F, F, F) {
    let t23685 = t362 * t1049;
    let t23696 = t23592 * t225;
    let t23712 = t23384 * t6787;
    let t23738 = t6818 * t3216;
    let t23742 = t1958 * t11094;
    let t23788 = t2752 * t28;
    (t23685, t23696, t23712, t23738, t23742, t23788)
}
