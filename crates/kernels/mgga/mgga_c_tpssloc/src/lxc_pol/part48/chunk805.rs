//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 805/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk805<F: Float>(t22986: F, t30623: F, t82159: F, t23185: F, t30634: F, t82074: F, t6662: F, t857: F, t23270: F, t776: F, t30667: F, t6547: F, t23222: F, t30663: F, t6552: F, t1880: F, t23196: F) -> (F, F, F, F, F, F) {
    let t112700 = 0.6579736267392905746e-1 * t22986 * t82159 * t30623;
    let t112702 = t23185 * t82074 * t30634;
    let t112703 = 0.3289868133696452873e-1 * t112702;
    let t112719 = t857 * t6662;
    let t112723 = 0.6579736267392905746e-1 * t22986 * t23270 * t112719 * t776;
    let t112726 = t6547 * t30667;
    let t112727 = 0.76763589786250567036e-1 * t112726;
    let t112730 = 0.3289868133696452873e-1 * t6552 * t30663 * t23222;
    let t112733 = 0.3289868133696452873e-1 * t1880 * t30663 * t23196;
    (t112700, t112703, t112723, t112727, t112730, t112733)
}
