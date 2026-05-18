//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 690/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk690<F: Float>(t1170: F, t1184: F, t1186: F, t19: F, t27: F, t498: F, t123: F, t497: F) -> (F, F, F, F, F, F) {
    let t3301 = t1170 * t1184;
    let t3302 = F::new(8.0) * t3301;
    let t3304 = F::new(8.0) * t1170 * t1186;
    let t3305 = t19 * t27;
    let t3307 = F::new(20.0) * t3305 * t498;
    let t3308 = t497 * t123;
    (t3301, t3302, t3304, t3305, t3307, t3308)
}
