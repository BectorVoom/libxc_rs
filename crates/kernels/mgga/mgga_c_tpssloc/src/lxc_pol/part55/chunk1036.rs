//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1036/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1036<F: Float>(t1338: F, t31181: F, t22674: F, t31123: F, t6897: F, t214: F, t6955: F, t2006: F, t794: F, t6907: F, t22724: F, t31127: F, t80645: F, t8458: F, t225: F, t31151: F) -> (F, F, F, F, F, F, F, F) {
    let t114130 = t1338 * t31181;
    let t114154 = t6897 * t22674 * t31123;
    let t114160 = t214 * t6955;
    let t114172 = t794 * t2006;
    let t114174 = t6897 * t114172 * t6907;
    let t114178 = 0.52089578783527170489e-1 * t22724 * t31127;
    let t114187 = t6897 * t80645 * t8458;
    let t114194 = t31151 * t225;
    (t114130, t114154, t114160, t114172, t114174, t114178, t114187, t114194)
}
