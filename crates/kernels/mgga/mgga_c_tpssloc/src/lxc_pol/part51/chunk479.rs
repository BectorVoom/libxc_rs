//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 479/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk479<F: Float>(t320: F, t941: F, t315: F, t2764: F, t2822: F, t323: F, t300: F, t938: F, t964: F, t969: F, t615: F, t972: F, t340: F, t697: F, t344: F, t221: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2903 = t941 * t320;
    let t2904 = 1.0 / t2903;
    let t2905 = t315 * t2904;
    let t2912 = 0.40256666666666666667e0 * t2764;
    let t2919 = 0.137975e0 * t2822;
    let t2928 = t941 * t941;
    let t2929 = 1.0 / t2928;
    let t2930 = t315 * t2929;
    let t2931 = t323 * t323;
    let t2932 = 1.0 / t2931;
    let t2940 = t300 * t938;
    let t2958 = t964 * t969;
    let t2960 = t615 * t972;
    let t2965 = t697 * t340;
    let t2966 = t2965 * t344;
    let t2967 = t221 * t2966;
    (t2904, t2905, t2912, t2919, t2929, t2930, t2932, t2940, t2958, t2960, t2965, t2966, t2967)
}
