//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2095/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2095<F: Float>(t35577: F, t1454: F, t2585: F, t2281: F, t4044: F, t4068: F, t92: F, t9384: F, t100: F, t9398: F, t1406: F, t9238: F) -> (F, F, F, F, F, F, F) {
    let t45496 = F::new(1.0) / t35577;
    let t45656 = t2585 * t1454;
    let t45658 = t2281 * t4044;
    let t45659 = F::new(22.0) / F::new(3.0) * t45658;
    let t45688 = t2281 * t4068;
    let t45689 = F::new(11.0) / F::new(3.0) * t45688;
    let t45697 = t92 * t9384;
    let t45707 = t100 * t9398;
    let t45844 = t1406 * t9238;
    (t45496, t45656, t45659, t45689, t45697, t45707, t45844)
}
