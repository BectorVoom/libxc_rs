//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1147/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1147<F: Float>(t22724: F, t31198: F, t22704: F, t22705: F, t31202: F, t22674: F, t31123: F, t6897: F, t214: F, t6955: F, t2006: F, t794: F, t6907: F, t31127: F, t80645: F, t8458: F) -> (F, F, F, F, F, F, F, F) {
    let t114119 = 0.52089578783527170489e-1 * t22724 * t31198;
    let t114121 = t22704 * t22705 * t31202;
    let t114154 = t6897 * t22674 * t31123;
    let t114160 = t214 * t6955;
    let t114172 = t794 * t2006;
    let t114174 = t6897 * t114172 * t6907;
    let t114178 = 0.52089578783527170489e-1 * t22724 * t31127;
    let t114187 = t6897 * t80645 * t8458;
    (t114119, t114121, t114154, t114160, t114172, t114174, t114178, t114187)
}
