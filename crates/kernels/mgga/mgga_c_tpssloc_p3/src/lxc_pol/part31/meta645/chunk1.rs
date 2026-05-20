//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1917/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1917<F: Float>(t1520: F, t254: F, t23270: F, t25038: F, t25039: F, t4119: F, t1880: F, t7488: F, t87782: F, t23237: F, t28276: F, t6552: F) -> (F, F, F, F) {
    let t98279 = t1520 * t254;
    let t98291 = t25038 * t23270 * t25039 * t4119;
    let t98305 = t1880 * t87782 * t7488;
    let t98315 = t6552 * t23237 * t28276;
    (t98279, t98291, t98305, t98315)
}
