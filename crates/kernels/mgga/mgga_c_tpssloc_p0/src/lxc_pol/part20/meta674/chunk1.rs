//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2543/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2543<F: Float>(t11269: F, t3313: F, t4785: F, t11191: F, t1670: F, t44075: F, t44077: F, t11403: F, t14838: F, t11407: F, t14850: F, t44159: F, t4745: F) -> (F, F, F, F, F) {
    let t51466 = F::cast_from(0.16081979498692535067e2_f64) * t3313 * t4785 * t11269;
    let t51470 = F::cast_from(0.24955700379505800916e5_f64) * t44075 * t1670 * t44077 * t11191;
    let t51472 = F::new(6.0) * t14838 * t11403;
    let t51474 = F::cast_from(0.48245938496077605201e2_f64) * t14850 * t11407;
    let t51476 = F::new(6.0) * t44159 * t4745;
    (t51466, t51470, t51472, t51474, t51476)
}
