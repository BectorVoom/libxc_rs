//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1960/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1960<F: Float>(t23993: F, t7428: F, t23998: F, t1860: F, t23992: F, t7445: F, t26028: F, t7032: F, t26016: F, t84173: F, t2032: F, t22534: F, t23970: F, t7782: F, t84237: F, t90098: F, t90101: F, t90104: F, t90132: F, t90137: F, t90153: F) -> F {
    let t91996 = t7428 * t23993;
    let t92001 = F::new(16.0) / F::new(9.0) * t7428 * t23998;
    let t92003 = t1860 * t23992 * t7445;
    let t92008 = F::new(16.0) / F::new(9.0) * t26028 * t7032;
    let t92012 = F::new(160.0) / F::new(9.0) * t26016 * t84173;
    let t92019 = -F::new(4.0) / F::new(3.0) * t90153 * t2032 + F::new(88.0) / F::new(27.0) * t91996 - F::new(2.0) / F::new(3.0) * t90132 * t2032 - t92001 + F::new(88.0) / F::new(27.0) * t92003 - F::new(2.0) / F::new(3.0) * t22534 * t7782 - t92008 - F::new(20.0) * t90137 * t84237 - t92012 + F::new(20.0) / F::new(3.0) * t90098 * t23970 + F::new(20.0) / F::new(3.0) * t90101 * t23970 + F::new(20.0) / F::new(3.0) * t90104 * t23970;
    t92019
}
