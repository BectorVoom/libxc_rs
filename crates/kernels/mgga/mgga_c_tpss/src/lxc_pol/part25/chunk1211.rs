//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1211/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1211<F: Float>(t19766: F, t6134: F, t45241: F, t782: F, t4716: F, t226: F, t44994: F, t4764: F, t818: F, t4799: F, t10584: F, t3664: F, t4783: F, t1378: F, t3721: F, t4759: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t70039 = t6134 * t19766;
    let t70042 = t45241 * t782;
    let t70046 = t4716 * t782;
    let t70060 = t44994 * t226;
    let t70063 = t4764 * t818;
    let t70070 = t4799 * t782 * t226;
    let t70074 = t4716 * t818;
    let t70094 = t10584 * t3664;
    let t70103 = t4783 * t782 * t226;
    let t70113 = t3721 * t1378 * t226;
    let t70123 = t4759 * t818;
    (t70039, t70042, t70046, t70060, t70063, t70070, t70074, t70094, t70103, t70113, t70123)
}
