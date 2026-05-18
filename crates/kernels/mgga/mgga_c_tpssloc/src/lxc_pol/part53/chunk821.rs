//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 821/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk821<F: Float>(t16036: F, t550: F, t6976: F, t1992: F, t16040: F, t7696: F, t794: F, t6897: F, t12461: F, t2094: F, t26163: F, t193: F, t200: F, t2056: F) -> (F, F, F, F, F, F) {
    let t26461 = t16036 * t550;
    let t26462 = t6976 * t26461;
    let t26463 = t1992 * t26462;
    let t26466 = t16040 * t550;
    let t26467 = t6976 * t26466;
    let t26468 = t1992 * t26467;
    let t26474 = t794 * t7696;
    let t26475 = t6897 * t26474;
    let t26558 = t2094 * t12461;
    let t26559 = t26558 * t26163;
    let t26563 = t193 * t200 * t2056;
    (t26463, t26468, t26475, t26558, t26559, t26563)
}
