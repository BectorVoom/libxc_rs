//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1189/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1189<F: Float>(t3270: F, t5992: F, t3274: F, t4721: F, t5973: F, t5977: F, t5981: F) -> (F, F) {
    let t5993 = t3270 * t5992;
    let t5999 = t3274 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4721 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5973 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5977 + t5981 / F::cast_from(3.0_f64);
    (t5993, t5999)
}
