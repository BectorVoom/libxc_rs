//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1442/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1442<F: Float>(t13278: F, t831: F, t2629: F, t4166: F, t4250: F, t9638: F, t4240: F, t4191: F, t2697: F, t4261: F, t820: F, t9645: F) -> (F, F, F, F, F, F, F) {
    let t13280 = F::new(7.0) / F::new(2304.0) * t13278 * t831;
    let t13283 = t4166 * t2629;
    let t13287 = F::new(7.0) / F::new(576.0) * t9638 * t4250;
    let t13320 = F::new(7.0) / F::new(2304.0) * t9638 * t4240;
    let t13330 = F::new(7.0) / F::new(576.0) * t9638 * t4191;
    let t13345 = F::new(7.0) / F::new(576.0) * t2697 * t4261;
    let t13350 = t9645 * t820;
    (t13280, t13283, t13287, t13320, t13330, t13345, t13350)
}
