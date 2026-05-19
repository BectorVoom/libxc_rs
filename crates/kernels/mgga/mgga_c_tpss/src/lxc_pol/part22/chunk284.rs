//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 284/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk284<F: Float>(t294: F, t305: F, t843: F, t869: F, t872: F, t877: F, t886: F, t892: F, t896: F, t905: F, t309: F) -> (F, F, F) {
    let t909 = t294 * (-F::new(0.310907e-1) * t872 * t305 + F::new(1.0) * t877 * t886 + t843 - t869 - F::cast_from(0.19751673498613801407e-1_f64) * t892 + F::cast_from(0.5848223622634646207e0_f64) * t896 * t905);
    let t911 = F::cast_from(0.19751673498613801407e-1_f64) * t294 * t892;
    let t912 = t294 * t309;
    (t909, t911, t912)
}
