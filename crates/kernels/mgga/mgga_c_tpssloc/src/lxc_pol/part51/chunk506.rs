//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 506/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk506<F: Float>(t1338: F, t1372: F, t193: F, t532: F, t1388: F, t1390: F, t112: F, t1395: F) -> (F, F, F, F) {
    let t3901 = t1338 * t1372;
    let t3918 = t193 * t532;
    let t3919 = t1388 * t1390;
    let t3938 = t1395 * t112;
    (t3901, t3918, t3919, t3938)
}
