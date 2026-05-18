//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1298/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1298<F: Float>(t31687: F, t9239: F, t31677: F, t131: F, t2240: F, t23966: F, t31684: F, t31680: F, t9231: F, t8511: F, t113875: F, t1862: F) -> (F, F, F, F, F, F, F, F) {
    let t115876 = t9239 * t31687;
    let t115877 = t115876 * t31677;
    let t115888 = t2240 * t23966 * t131;
    let t115889 = t115888 * t31684;
    let t115891 = t9231 * t31680;
    let t115894 = t8511 * t131;
    let t115895 = t9239 * t115894;
    let t115903 = t113875 * t1862;
    (t115876, t115877, t115888, t115889, t115891, t115894, t115895, t115903)
}
