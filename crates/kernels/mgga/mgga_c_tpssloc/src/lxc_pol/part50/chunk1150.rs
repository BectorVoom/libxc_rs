//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1150/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1150<F: Float>(t6703: F, t6768: F, t30920: F, t3216: F, t11094: F, t8409: F, t43637: F, t8413: F, t31003: F, t39054: F, t31016: F, t9231: F) -> (F, F, F, F, F, F) {
    let t113619 = t6703 * t6768;
    let t113633 = t30920 * t3216;
    let t113637 = t8409 * t11094;
    let t113655 = t8413 * t43637;
    let t113845 = t39054 * t31003;
    let t113848 = t9231 * t31016;
    (t113619, t113633, t113637, t113655, t113845, t113848)
}
