//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1039/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1039<F: Float>(t4866: F, t673: F, t4869: F, t4872: F, t4831: F, t664: F) -> (F, F, F, F) {
    let t14497 = t673 * t4866;
    let t14501 = t673 * t4869;
    let t14503 = t673 * t4872;
    let t14505 = t664 * t4831;
    (t14497, t14501, t14503, t14505)
}
