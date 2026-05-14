//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 674/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk674<F: Float>(t22705: F, t7736: F, t22704: F, t6883: F, t7741: F, t7740: F, t794: F, t6897: F, t1338: F, t7722: F, t7696: F, t225: F, t7704: F, t112: F, t7758: F, t12461: F, t2094: F) -> (F, F, F, F, F, F, F, F) {
    let t26426 = t22705 * t7736;
    let t26427 = t22704 * t26426;
    let t26429 = t6883 * t7741;
    let t26436 = t794 * t7740;
    let t26437 = t6897 * t26436;
    let t26458 = t1338 * t7722;
    let t26474 = t794 * t7696;
    let t26475 = t6897 * t26474;
    let t26477 = t7704 * t225;
    let t26523 = t7758 * t112;
    let t26558 = t2094 * t12461;
    (t26427, t26429, t26437, t26458, t26475, t26477, t26523, t26558)
}
