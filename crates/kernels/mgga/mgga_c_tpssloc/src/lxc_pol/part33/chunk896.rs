//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 896/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk896<F: Float>(t4997: F, t5002: F, t11784: F, t248: F, t5971: F, t1227: F, t5019: F, t4993: F, t5005: F, t5024: F, t1017: F, t6163: F) -> (F, F, F, F, F, F) {
    let t18972 = t5002 * t4997;
    let t18975 = t248 * t11784 * t5971;
    let t18976 = t1227 * t18975;
    let t18978 = t5019 * t4997;
    let t18980 = t5005 * t4993;
    let t18987 = t5024 * t4993;
    let t19024 = t6163 * t1017;
    (t18972, t18976, t18978, t18980, t18987, t19024)
}
