//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1317/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1317<F: Float>(t33211: F, t6535: F, t191: F, t192: F, t26138: F, t2020: F, t33137: F, t6876: F, t22574: F, t25988: F, t36533: F, t25985: F, t8450: F) -> (F, F, F, F, F) {
    let t120069 = F::new(4.0) * t33211 * t6535;
    let t120071 = t26138 * t191 * t192;
    let t120072 = t120071 * t2020;
    let t120075 = F::new(2.0) * t6876 * t33137;
    let t120078 = F::new(6.0) * t22574 * t36533 * t25988;
    let t120079 = t8450 * t25985;
    (t120069, t120072, t120075, t120078, t120079)
}
