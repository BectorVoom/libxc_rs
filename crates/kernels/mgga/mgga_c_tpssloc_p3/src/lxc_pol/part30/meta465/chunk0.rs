//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1750/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1750<F: Float>(t210: F, t6679: F, t3139: F, t6717: F, t3113: F, t6754: F, t3107: F, t6753: F, t1012: F, t1933: F, t607: F, t1937: F) -> (F, F, F, F, F, F) {
    let t23422 = t6679 * t210;
    let t23425 = t6717 * t3139;
    let t23433 = t3113 * t6754;
    let t23436 = t6753 * t3107;
    let t23437 = t1012 * t23436;
    let t23442 = t1933 * t607;
    let t23443 = t23442 * t1937;
    (t23422, t23425, t23433, t23436, t23437, t23443)
}
