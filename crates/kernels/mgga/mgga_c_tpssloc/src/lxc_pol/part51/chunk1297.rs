//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1297/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1297<F: Float>(t1862: F, t8308: F, t31688: F, t31693: F, t31687: F, t8515: F, t9231: F, t31019: F, t2240: F, t240: F, t8301: F, t39054: F, t8511: F) -> (F, F, F, F, F, F) {
    let t115833 = t8308 * t1862;
    let t115837 = t31688 * t31693;
    let t115846 = t9231 * t31687 * t8515;
    let t115853 = t31688 * t31019;
    let t115860 = F::new(55.0) / F::new(81.0) * t2240 * t8301 * t240 * t8515;
    let t115866 = t39054 * t8511;
    (t115833, t115837, t115846, t115853, t115860, t115866)
}
