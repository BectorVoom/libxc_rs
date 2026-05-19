//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 918/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk918<F: Float>(t498: F, t9904: F, t1170: F, t3197: F, t1186: F, t3214: F, t30: F, t490: F, t33: F, t493: F, t1193: F, t8115: F) -> (F, F, F, F, F, F) {
    let t9906 = F::new(120.0) * t9904 * t498;
    let t9907 = t1170 * t3197;
    let t9913 = t3214 * t1186;
    let t9922 = t30 * t30;
    let t9924 = F::new(1.0) / t490 / t9922;
    let t9934 = t33 * t33;
    let t9936 = F::new(1.0) / t493 / t9934;
    let t9954 = F::cast_from(0.51947577317044391277e2_f64) * t1193 * t8115;
    (t9906, t9907, t9913, t9924, t9936, t9954)
}
