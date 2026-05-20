//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 561/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk561<F: Float>(t1239: F, t496: F, t68: F, t3032: F, t3502: F, t3499: F) -> (F, F, F) {
    let t3597 = F::new(1.0) / t1239 / t496;
    let t3598 = t68 * t3597;
    let t3609 = t3032 * t3502;
    let t3610 = t3499 * t3609;
    (t3598, t3609, t3610)
}
