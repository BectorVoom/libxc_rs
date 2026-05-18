//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 976/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk976<F: Float>(t23909: F, t8526: F, t23918: F, t1873: F, t24428: F, t652: F, t2018: F, t26161: F, t26558: F, t3914: F, t23938: F, t6535: F) -> (F, F, F, F, F) {
    let t115210 = F::new(2.0) * t8526 * t23909;
    let t115212 = F::new(2.0) * t8526 * t23918;
    let t115217 = F::new(2.0) * t652 * t24428 * t1873;
    let t115227 = F::new(2.0) * t26161 * t26558 * t2018 * t3914;
    let t115229 = F::new(4.0) * t23938 * t6535;
    (t115210, t115212, t115217, t115227, t115229)
}
