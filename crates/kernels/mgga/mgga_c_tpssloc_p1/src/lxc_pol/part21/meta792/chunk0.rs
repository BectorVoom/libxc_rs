//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2752/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2752<F: Float>(t46244: F, t185: F, t2658: F, t55723: F, t152: F, t2244: F, t5499: F, t4303: F, t868: F, t12892: F, t16693: F, t16616: F, t2535: F) -> (F, F, F, F, F, F) {
    let t57996 = F::new(8.0) * t46244;
    let t58005 = F::new(24.0) * t2658 * t185 * t55723;
    let t58008 = F::new(24.0) * t2244 * t152 * t5499;
    let t58009 = t4303 * t868;
    let t58020 = F::new(24.0) * t16693 * t12892;
    let t58021 = t16616 * t2535;
    (t57996, t58005, t58008, t58009, t58020, t58021)
}
