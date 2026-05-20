//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1420/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1420<F: Float>(t1226: F, t4965: F, t11697: F, t4953: F, t3577: F, t1229: F, t3242: F, t13969: F, t4979: F, t3506: F, t4973: F, t1227: F) -> (F, F, F, F, F, F, F, F) {
    let t15594 = t4965 * t1226;
    let t15608 = t11697 * t4953;
    let t15610 = t3577 * t15608 / F::new(3456.0);
    let t15615 = t1229 * t3242;
    let t15640 = t13969 * t4979;
    let t15642 = t3506 * t15640 / F::new(1152.0);
    let t15643 = t13969 * t4973;
    let t15645 = t1227 * t15643 / F::new(1728.0);
    (t15594, t15608, t15610, t15615, t15640, t15642, t15643, t15645)
}
