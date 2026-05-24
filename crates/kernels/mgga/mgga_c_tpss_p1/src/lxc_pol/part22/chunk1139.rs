//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1139/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1139<F: Float>(t30: F, t12691: F, t489: F, t1288: F, t9856: F, t2: F, t3282: F, t555: F, t580: F, t1991: F, t22: F, t3218: F, t4360: F, t4363: F, t490: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t12692 = t489 * t12691;
    let t12696 = t9856 * t1288;
    let t12699 = t3282 * t2;
    let t12700 = t555 * t580;
    let t12710 = piecewise3::<F>(t31, F::new(0.0), -F::new(8.0) / F::new(27.0) * t12696 * t3218 + F::new(16.0) / F::new(9.0) * t12699 * t12700 + F::new(4.0) / F::new(9.0) * t4360 * t1991 + F::new(8.0) / F::new(3.0) * t490 * t555 - F::new(8.0) * t4363 * t22);
    (t12692, t12700, t12710)
}
