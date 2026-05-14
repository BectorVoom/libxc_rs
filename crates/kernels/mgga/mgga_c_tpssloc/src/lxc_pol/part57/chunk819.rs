//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 819/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk819<F: Float>(t2240: F, t240: F, t8301: F, t8515: F, t39063: F, t8511: F, t31687: F, t9239: F, t131: F, t23966: F, t113875: F, t1862: F, t31680: F, t22573: F, t8606: F, t111: F, t8646: F) -> (F, F, F, F, F, F, F, F, F) {
    let t115860 = 55.0 / 81.0 * t2240 * t8301 * t240 * t8515;
    let t115871 = t39063 * t8511;
    let t115876 = t9239 * t31687;
    let t115888 = t2240 * t23966 * t131;
    let t115894 = t8511 * t131;
    let t115895 = t9239 * t115894;
    let t115903 = t113875 * t1862;
    let t115907 = t9239 * t31680;
    let t115925 = t8606 * t22573;
    let t115984 = t8646 * t111;
    (t115860, t115871, t115876, t115888, t115895, t115903, t115907, t115925, t115984)
}
