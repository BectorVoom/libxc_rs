//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 901/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk901<F: Float>(t30635: F, t6579: F, t1888: F, t23270: F, t25169: F, t2719: F, t22986: F, t30623: F, t82159: F, t23185: F, t30634: F, t82074: F) -> (F, F, F, F) {
    let t112686 = t6579 * t30635;
    let t112687 = F::cast_from(0.15352717957250113407e0_f64) * t112686;
    let t112697 = F::cast_from(0.9869604401089358619e-1_f64) * t1888 * t23270 * t25169 * t2719;
    let t112700 = F::cast_from(0.6579736267392905746e-1_f64) * t22986 * t82159 * t30623;
    let t112702 = t23185 * t82074 * t30634;
    (t112687, t112697, t112700, t112702)
}
