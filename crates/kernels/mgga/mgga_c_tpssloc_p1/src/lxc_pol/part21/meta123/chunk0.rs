//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 829/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk829<F: Float>(t2988: F, t2990: F, t2775: F, t344: F, t2244: F, t977: F, t2250: F, t978: F, t2822: F, t2824: F, t2828: F, t2831: F, t2834: F) -> (F, F, F, F, F, F, F, F) {
    let t2991 = t2988 * t2990;
    let t2994 = t344 * t2775;
    let t2995 = t2994 * t2244;
    let t2996 = t977 * t2995;
    let t2999 = t978 * t2250;
    let t3000 = t977 * t2999;
    let t3003 = F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t2822;
    let t3008 = -t3003 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2824 + t2828 / F::cast_from(18.0_f64) - t2831 / F::cast_from(3.0_f64) + t2834 / F::cast_from(6.0_f64);
    (t2991, t2994, t2995, t2996, t2999, t3000, t3003, t3008)
}
