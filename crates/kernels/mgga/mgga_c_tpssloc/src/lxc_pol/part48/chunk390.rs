//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 390/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk390<F: Float>(t1043: F, t154: F, t632: F, t2244: F, t123: F) -> (F, F, F) {
    let t2768 = t154 * t1043;
    let t2769 = t632 * t632;
    let t2770 = 1.0 / t2769;
    let t2771 = t2770 * t2244;
    let t2772 = t2768 * t2771;
    let t2773 = t123 * t2772;
    (t2770, t2771, t2773)
}
