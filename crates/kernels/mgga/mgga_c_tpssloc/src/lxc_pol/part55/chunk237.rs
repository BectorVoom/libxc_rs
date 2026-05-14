//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 237/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk237<F: Float>(t906: F, t241: F, t340: F, t884: F, t136: F, t886: F, t897: F, t899: F, t902: F, t290: F, t893: F, t880: F, t307: F, t302: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t907 = 0.82156666666666666667e-1 * t906;
    let t908 = t241 * t340;
    let t909 = t908 * t884;
    let t910 = t136 * t909;
    let t912 = 0.1898925e1 * t897 - t899 - 0.29896666666666666667e0 * t886 + 0.3071625e0 * t902 - t907 - 0.82156666666666666667e-1 * t910;
    let t913 = 1.0 / t290;
    let t914 = t912 * t913;
    let t916 = 1.0 * t893 * t914;
    let t917 = 0.17123333333333333333e-1 * t880;
    let t919 = -t917 - 0.17123333333333333333e-1 * t886;
    let t922 = t307 * t307;
    let t923 = 1.0 / t922;
    let t924 = t302 * t923;
    let t926 = 0.516475e0 * t880;
    let t929 = 0.104195e0 * t906;
    let t931 = 0.3529725e1 * t897 - t926 - 0.516475e0 * t886 + 0.6311625e0 * t902 - t929 - 0.104195e0 * t910;
    (t907, t908, t909, t910, t912, t913, t914, t916, t917, t919, t922, t923, t924, t926, t929, t931)
}
