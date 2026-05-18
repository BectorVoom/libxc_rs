//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 890/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk890<F: Float>(t31366: F, t7479: F, t6552: F, t7488: F, t1880: F, t225: F, t258: F, t7823: F, t214: F, t1911: F, t7841: F, t2718: F) -> (F, F, F, F, F, F, F, F) {
    let t33419 = t31366 * t7479;
    let t33420 = t6552 * t33419;
    let t33422 = t31366 * t7488;
    let t33423 = t1880 * t33422;
    let t33428 = t7823 * t225 * t258;
    let t33429 = t214 * t33428;
    let t33430 = t1880 * t33429;
    let t33432 = t7841 * t1911;
    let t33433 = t2718 * t33432;
    (t33419, t33420, t33422, t33423, t33428, t33429, t33430, t33433)
}
