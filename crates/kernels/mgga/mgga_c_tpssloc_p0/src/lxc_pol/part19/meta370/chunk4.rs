//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1375/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1375<F: Float>(t3209: F, t3213: F, t3215: F, t193: F, t3216: F, t336: F, t41992: F, t41998: F, t42002: F, t42005: F, t42025: F, t42031: F, t42097: F, t42105: F, t42682: F, t42686: F, t42688: F) -> F {
    let t43629 = t3209 * t3209;
    let t43634 = t3213 * t3213;
    let t43636 = t3215 * t3215;
    let t43637 = F::new(1.0) / t43636;
    let t43641 = -F::new(3.0) * t193 * t3216 * t336 * t43629 - F::new(6.0) * t193 * t336 * t43634 * t43637 + t41992 - t41998 - t42002 + t42005 + t42025 - t42031 + t42097 + t42105 - t42682 + t42686 - t42688;
    t43641
}
