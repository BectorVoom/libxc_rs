//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 855/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk855<F: Float>(t3886: F, t7936: F, t33246: F, t6883: F, t214: F, t7918: F, t225: F, t33259: F, t22704: F, t33249: F, t81326: F, t33297: F, t22674: F, t33296: F, t6897: F, t22751: F, t33307: F) -> (F, F, F, F, F, F, F, F) {
    let t122142 = t3886 * t7936;
    let t122152 = t6883 * t33246;
    let t122166 = t214 * t7918;
    let t122172 = t33259 * t225;
    let t122178 = t22704 * t81326 * t33249;
    let t122210 = t6883 * t33297;
    let t122247 = t6897 * t22674 * t33296;
    let t122251 = t22751 * t33307;
    (t122142, t122152, t122166, t122172, t122178, t122210, t122247, t122251)
}
