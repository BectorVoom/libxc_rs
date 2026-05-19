//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 888/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk888<F: Float>(t1998: F, t2006: F, t214: F, t1985: F, t553: F, t8470: F, t544: F) -> (F, F, F, F) {
    let t8479 = t1998 * t2006;
    let t8480 = t214 * t8479;
    let t8482 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t8480;
    let t8483 = t553 * t8470;
    let t8485 = t544 * t8483 + t8482;
    (t8479, t8480, t8483, t8485)
}
