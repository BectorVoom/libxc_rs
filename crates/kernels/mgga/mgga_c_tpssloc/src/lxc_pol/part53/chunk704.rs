//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 704/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk704<F: Float>(t10143: F, t2056: F, t2094: F, t3701: F, t112: F, t7222: F, t111: F, t2098: F, t191: F, t192: F, t5118: F, t1390: F, t5187: F, t531: F, t1982: F, t25: F, t870: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24344 = t2056 * t10143;
    let t24432 = t2094 * t3701;
    let t24462 = t7222 * t112;
    let t24465 = t2098 * t111;
    let t24987 = t5118 * t191 * t192;
    let t24990 = t1390 * t5187;
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    let t25014 = t870 * t25;
    (t24344, t24432, t24462, t24465, t24987, t24990, t24994, t24995, t25014)
}
