//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1190/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1190<F: Float>(t2793: F, t10661: F, t913: F, t2836: F, t2792: F, t2842: F, t2844: F, t2880: F, t2897: F, t2904: F, t10701: F, t888: F, t10705: F, t275: F, t2790: F, t2840: F) -> (F, F, F, F, F, F, F, F) {
    let t41995 = t2793 * t2793;
    let t41998 = 24.0 * t10661 * t41995 * t913;
    let t41999 = t2836 * t2836;
    let t42002 = 6.0 * t2792 * t41999 * t913;
    let t42005 = 0.48245938496077605201e2 * t2842 * t41999 * t2844;
    let t42011 = t2880 * t2880;
    let t42020 = t2897 * t2904;
    let t42023 = t888 * t10701;
    let t42025 = 0.2069040516770936012e4 * t42023 * t10705;
    let t42028 = t275 / t2840 / t2790;
    (t41995, t41998, t42002, t42005, t42011, t42020, t42025, t42028)
}
