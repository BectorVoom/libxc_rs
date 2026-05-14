//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1163/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1163<F: Float>(t31019: F, t31688: F, t2240: F, t240: F, t8301: F, t8515: F, t39054: F, t8511: F, t31687: F, t9239: F, t31677: F, t131: F, t23966: F, t31684: F, t31680: F, t9231: F) -> (F, F, F, F, F, F, F, F) {
    let t115853 = t31688 * t31019;
    let t115860 = 55.0 / 81.0 * t2240 * t8301 * t240 * t8515;
    let t115866 = t39054 * t8511;
    let t115876 = t9239 * t31687;
    let t115877 = t115876 * t31677;
    let t115888 = t2240 * t23966 * t131;
    let t115889 = t115888 * t31684;
    let t115891 = t9231 * t31680;
    (t115853, t115860, t115866, t115876, t115877, t115888, t115889, t115891)
}
