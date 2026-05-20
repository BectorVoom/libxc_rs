//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2153/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2153<F: Float>(t2631: F, t47285: F, t6605: F, t9972: F, t12971: F, t1894: F, t236: F, t6591: F, t23046: F, t4184: F, t812: F, t836: F) -> (F, F, F) {
    let t87355 = t6605 * t9972 * t47285 * t2631;
    let t87359 = t6591 * t1894 * t236 * t12971;
    let t87363 = t812 * t23046 * t836 * t4184;
    (t87355, t87359, t87363)
}
