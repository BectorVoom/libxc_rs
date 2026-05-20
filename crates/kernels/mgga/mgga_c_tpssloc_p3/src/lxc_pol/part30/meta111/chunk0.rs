//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 676/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk676<F: Float>(t154: F, t2559: F, t222: F, t2563: F, t805: F, t68: F, t808: F) -> (F, F, F, F) {
    let t2600 = t2559 * t154;
    let t2602 = F::new(35.0) / F::new(432.0) * t2600 * t222;
    let t2603 = t2563 * t805;
    let t2617 = t808 * t68;
    (t2600, t2602, t2603, t2617)
}
