//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1019/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1019<F: Float>(t1887: F, t22797: F, t268: F, t547: F, t6559: F, t225: F, t22643: F, t23069: F, t229: F, t23228: F, t794: F, t852: F, t213: F, t221: F, t697: F, t111: F, t7415: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t81159 = t22797 * t1887;
    let t81228 = t6559 * t547 * t268;
    let t81326 = t22643 * t225;
    let t81591 = t23069 * t1887;
    let t81651 = t6559 * t229 * t268;
    let t82074 = t23228 * t225;
    let t82133 = t794 * t852;
    let t82159 = t213 * t852 * t225;
    let t82631 = t221 * t697;
    let t85416 = t7415 * t111;
    (t81159, t81228, t81326, t81591, t81651, t82074, t82133, t82159, t82631, t85416)
}
