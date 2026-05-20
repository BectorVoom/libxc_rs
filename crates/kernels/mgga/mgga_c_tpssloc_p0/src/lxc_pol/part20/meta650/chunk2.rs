//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2392/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2392<F: Float>(t48907: F, t48920: F, t48934: F, t48990: F, t49004: F, t49026: F, t49042: F, t49062: F, t893: F, t913: F, t14388: F, t2836: F, t2842: F) -> (F, F) {
    let t49068 = F::new(1.0) * t893 * (t48907 + t48920 + t48934 + t48990 + t49004 + t49026 + t49042 + t49062) * t913;
    let t49071 = F::cast_from(0.48245938496077605201e2_f64) * t2842 * t14388 * t2836;
    (t49068, t49071)
}
