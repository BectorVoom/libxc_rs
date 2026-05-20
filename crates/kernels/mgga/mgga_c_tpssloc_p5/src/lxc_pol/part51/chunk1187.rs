//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1187/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1187<F: Float>(t31668: F, t533: F, t1390: F, t1983: F, t8511: F, t9231: F, t9239: F, t645: F, t8513: F, t8514: F, t131: F, t7025: F) -> (F, F, F, F, F, F, F) {
    let t31669 = t533 * t31668;
    let t31670 = t31669 * t1390;
    let t31671 = t1983 * t31670;
    let t31672 = t9231 * t8511;
    let t31675 = t9239 * t8511;
    let t31677 = t8513 * t8514 * t645;
    let t31680 = t7025 * t131;
    (t31669, t31670, t31671, t31672, t31675, t31677, t31680)
}
