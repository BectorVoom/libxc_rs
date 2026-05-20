//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1911/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1911<F: Float>(t10109: F, t1888: F, t23270: F, t5636: F, t865: F, t25045: F, t86873: F, t214: F, t5631: F, t1880: F, t6572: F, t22986: F, t5657: F, t776: F, t857: F) -> (F, F, F, F, F) {
    let t98122 = t1888 * t23270 * t10109 * t5636 * t865;
    let t98125 = t1888 * t86873 * t25045;
    let t98133 = t214 * t5631;
    let t98135 = t1880 * t98133 * t6572;
    let t98148 = t22986 * t23270 * t857 * t5657 * t776;
    (t98122, t98125, t98133, t98135, t98148)
}
