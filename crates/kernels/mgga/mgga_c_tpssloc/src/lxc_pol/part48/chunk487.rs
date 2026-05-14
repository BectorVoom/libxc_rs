//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 487/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk487<F: Float>(t109: F, t652: F, t6525: F, t107: F, t625: F, t63: F, t656: F, t666: F) -> (F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t6527 = 2.0 * t652 * t6525;
    let t6528 = t625 * t107;
    let t6529 = t6528 / 3.0;
    let t6530 = t63 * t656;
    let t6531 = t6530 * t666;
    let t6534 = piecewise3(t110, 0.0, -t6529 - t6531 / 8.0);
    (t6527, t6528, t6530, t6531, t6534)
}
