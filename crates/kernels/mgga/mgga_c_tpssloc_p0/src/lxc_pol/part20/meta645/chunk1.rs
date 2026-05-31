//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2369/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2369<F: Float>(t1041: F, t13969: F, t14142: F, t14179: F, t10309: F, t10408: F, t14126: F, t14167: F, t1616: F, t2776: F, t3070: F, t3071: F, t3117: F, t42478: F, t42481: F, t42490: F, t42546: F, t43358: F, t4579: F, t4582: F, t4650: F, t47779: F, t47915: F, t48260: F, t48497: F, t48607: F) -> F {
    let t48626 = t1041 * t13969 * t14142;
    let t48629 = t1041 * t13969 * t14179;
    let t48656 = t3117 * t14167 / F::cast_from(256.0_f64) - t48626 / F::cast_from(576.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t48629 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t1041 * t4582 * t47779 * t48497 - t3070 * t3071 * t4650 * t2776 / F::cast_from(768.0_f64) + t48607 * t3071 * t47915 / F::cast_from(256.0_f64) - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t48607 * t10408 * t48260 - F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t3070 * t10408 * t1616 * t10309 + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t43358 * t4579 - t42478 / F::cast_from(2304.0_f64) + t42481 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t42490 - t42546 * t14126 / F::cast_from(1536.0_f64);
    t48656
}
