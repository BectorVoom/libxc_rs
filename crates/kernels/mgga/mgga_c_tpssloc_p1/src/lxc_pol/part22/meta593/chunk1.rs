//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2110/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2110<F: Float>(t242: F, t41347: F, t812: F, t2627: F, t4265: F, t226: F, t40931: F, t68: F, t2394: F, t4344: F) -> (F, F, F, F) {
    let t47307 = t812 * t41347 * t242;
    let t47374 = t2627 * t4265;
    let t47386 = t226 * t68 * t40931;
    let t47705 = t2394 * t4344;
    (t47307, t47374, t47386, t47705)
}
