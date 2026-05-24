//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 750/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk750<F: Float>(t5012: F, t1482: F, t2776: F, t366: F, t4977: F, t1464: F, t1474: F, t4988: F, t220: F, t2782: F, t2786: F, t2798: F, t2799: F, t368: F, t983: F, t985: F, param_beta: F) -> (F, F, F, F, F, F, F) {
    let t5013 = param_beta * t5012;
    let t5017 = t1482 * t1482;
    let t5018 = t2776 * t5017;
    let t5021 = t366 * t4977;
    let t5025 = t1474 * t1464;
    let t5029 = t366 * t4988;
    let t5036 = t220 * t368 * t5012 + F::new(2.0) * t2782 * t2786 * t5021 - t2798 * t2799 * t5021 + F::new(2.0) * t5025 * t983 * t985 + t5029 * t983 * t985;
    (t5013, t5017, t5018, t5021, t5025, t5029, t5036)
}
