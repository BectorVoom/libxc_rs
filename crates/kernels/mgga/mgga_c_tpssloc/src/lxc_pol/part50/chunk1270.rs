//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1270/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1270<F: Float>(t32761: F, t6897: F, t794: F, t114208: F, t114216: F, t114285: F, t1992: F, t26355: F, t114240: F, t114242: F, t114172: F, t7700: F) -> (F, F, F, F, F, F, F) {
    let t120550 = t6897 * t794 * t32761;
    let t120551 = F::new(0.82246703342411321825e-2) * t120550;
    let t120552 = F::new(0.76763589786250567036e-1) * t114208;
    let t120553 = F::new(0.76763589786250567036e-1) * t114216;
    let t120556 = F::new(0.3289868133696452873e-1) * t1992 * t114285 * t26355;
    let t120561 = F::new(0.16449340668482264365e-1) * t114240;
    let t120566 = F::new(0.38381794893125283518e-1) * t114242;
    let t120568 = t6897 * t114172 * t7700;
    (t120551, t120552, t120553, t120556, t120561, t120566, t120568)
}
