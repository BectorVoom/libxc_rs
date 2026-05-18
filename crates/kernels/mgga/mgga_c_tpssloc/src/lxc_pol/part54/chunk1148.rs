//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1148/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1148<F: Float>(t2006: F, t552: F, t1307: F, t6637: F, t6888: F, t794: F, t8479: F, t6897: F, t1351: F, t550: F, t6976: F, t1992: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31193 = t552 * t2006;
    let t31194 = t31193 * t1307;
    let t31195 = t6637 * t31194;
    let t31197 = F::new(0.3289868133696452873e-1) * t6888 * t31195;
    let t31198 = t794 * t8479;
    let t31200 = F::new(0.82246703342411321825e-2) * t6897 * t31198;
    let t31201 = t2006 * t1351;
    let t31202 = t31201 * t550;
    let t31203 = t6976 * t31202;
    let t31205 = F::new(0.16449340668482264365e-1) * t1992 * t31203;
    (t31193, t31194, t31195, t31197, t31198, t31200, t31202, t31203, t31205)
}
