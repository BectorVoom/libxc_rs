//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1166/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1166<F: Float>(t118466: F, t22960: F, t7537: F, t857: F, t22986: F, t23270: F, t776: F, t32814: F, t82159: F, t32815: F, t81591: F, t112899: F, t1888: F, t25045: F) -> (F, F, F, F, F) {
    let t118467 = t22960 * t118466;
    let t118472 = t857 * t7537;
    let t118476 = F::new(0.3289868133696452873e-1) * t22986 * t23270 * t118472 * t776;
    let t118479 = F::new(0.3289868133696452873e-1) * t22986 * t82159 * t32814;
    let t118480 = t81591 * t32815;
    let t118481 = F::new(0.76763589786250567037e-1) * t118480;
    let t118484 = F::new(0.3289868133696452873e-1) * t1888 * t112899 * t25045;
    (t118467, t118476, t118479, t118481, t118484)
}
