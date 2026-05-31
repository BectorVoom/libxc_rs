//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1742/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1742<F: Float>(t776: F, t828: F, t13228: F, t13222: F, t1500: F, t2693: F, t4163: F, t838: F, t120: F, t4233: F) -> (F, F, F, F, F) {
    let t13229 = t828 * t776;
    let t13230 = t13228 * t13229;
    let t13231 = t13222 * t13230;
    let t13234 = t1500 * t2693;
    let t13237 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t4163 * t838;
    let t13242 = t120 * t4233;
    (t13229, t13231, t13234, t13237, t13242)
}
