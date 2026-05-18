//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 948/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk948<F: Float>(t11873: F, t11875: F, t4057: F, t664: F, t1023: F, t4060: F, t1505: F, t2910: F, t294: F, t4155: F, t11844: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11940 = F::new(4.0) / F::new(27.0) * t11873;
    let t11941 = F::new(4.0) / F::new(9.0) * t11875;
    let t11942 = t664 * t4057;
    let t11943 = F::new(2.0) / F::new(9.0) * t11942;
    let t11958 = F::new(0.19931111111111111111e0) * t11942;
    let t11971 = t4060 * t1023;
    let t11976 = t1505 * t2910;
    let t11988 = F::new(0.41203703703703703704e-2) * t11873;
    let t11989 = F::new(0.12361111111111111111e-1) * t11875;
    let t11990 = F::new(0.61805555555555555556e-2) * t11942;
    let t12009 = t294 * t4155;
    let t12024 = F::new(0.13892666666666666667e0) * t11844;
    (t11940, t11941, t11942, t11943, t11958, t11971, t11976, t11988, t11989, t11990, t12009, t12024)
}
