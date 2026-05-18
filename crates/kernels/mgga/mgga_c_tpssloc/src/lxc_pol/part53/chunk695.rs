//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 695/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk695<F: Float>(t2047: F, t225: F, t258: F, t214: F, t1880: F, t2053: F, t6571: F) -> (F, F, F, F) {
    let t8537 = t2047 * t225 * t258;
    let t8538 = t214 * t8537;
    let t8539 = t1880 * t8538;
    let t8547 = t6571 * t2053;
    (t8537, t8538, t8539, t8547)
}
