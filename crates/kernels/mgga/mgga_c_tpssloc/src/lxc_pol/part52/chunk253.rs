//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 253/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk253<F: Float>(t886: F, t917: F, t307: F, t302: F, t880: F, t906: F, t897: F, t902: F, t910: F, t310: F, t324: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t919 = -t917 - F::new(0.17123333333333333333e-1) * t886;
    let t922 = t307 * t307;
    let t923 = F::new(1.0) / t922;
    let t924 = t302 * t923;
    let t926 = F::new(0.516475e0) * t880;
    let t929 = F::new(0.104195e0) * t906;
    let t931 = F::new(0.3529725e1) * t897 - t926 - F::new(0.516475e0) * t886 + F::new(0.6311625e0) * t902 - t929 - F::new(0.104195e0) * t910;
    let t932 = F::new(1.0) / t310;
    let t933 = t931 * t932;
    let t936 = F::new(0.92708333333333333333e-2) * t880;
    let t938 = -t936 - F::new(0.92708333333333333333e-2) * t886;
    let t939 = t938 * t324;
    (t919, t922, t923, t924, t926, t929, t931, t932, t933, t936, t938, t939)
}
