//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1175/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1175<F: Float>(t24829: F, t462: F, t7319: F, t7327: F, t7377: F, t2144: F, t3507: F, t3625: F, t1215: F, t7348: F, t1246: F, t1170: F, t7381: F) -> (F, F, F, F, F, F, F) {
    let t24830 = t462 * t24829;
    let t24833 = t7319 * t7327;
    let t24834 = t24833 * t7377;
    let t24837 = t2144 * t3507;
    let t24838 = t24837 * t3625;
    let t24840 = t7348 * t1215;
    let t24841 = t24840 * t1246;
    let t24844 = t1170 * t7381;
    (t24830, t24833, t24834, t24837, t24838, t24841, t24844)
}
