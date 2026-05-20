//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 273/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk273<F: Float>(t912: F, t913: F, t893: F, t880: F, t886: F, t307: F, t302: F, t906: F, t897: F, t902: F, t910: F, t310: F) -> (F, F, F, F, F, F, F, F) {
    let t914 = t912 * t913;
    let t916 = F::new(1.0) * t893 * t914;
    let t917 = F::cast_from(0.17123333333333333333e-1_f64) * t880;
    let t919 = -t917 - F::cast_from(0.17123333333333333333e-1_f64) * t886;
    let t922 = t307 * t307;
    let t923 = F::new(1.0) / t922;
    let t924 = t302 * t923;
    let t926 = F::new(0.516475e0) * t880;
    let t929 = F::new(0.104195e0) * t906;
    let t931 = F::new(0.3529725e1) * t897 - t926 - F::new(0.516475e0) * t886 + F::new(0.6311625e0) * t902 - t929 - F::new(0.104195e0) * t910;
    let t932 = F::new(1.0) / t310;
    (t914, t916, t919, t922, t923, t924, t931, t932)
}
