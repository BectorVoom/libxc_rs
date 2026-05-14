//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 984/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk984<F: Float>(t2535: F, t3691: F, t12121: F, t12123: F, t12125: F, t12128: F, t12131: F, t12133: F, t12135: F, t12137: F, t12139: F, t12141: F, t9853: F, t9859: F, t12049: F, t12095: F, t12119: F, t225: F) -> (F, F) {
    let t12142 = t3691 * t2535;
    let t12143 = 0.17544670867903938621e1 * t12142;
    let t12144 = t12121 + t12123 + t12125 + t12128 + t12131 + t12133 - t12135 + t12137 + t9853 + t12139 + t9859 - t12141 - t12143;
    let t12147 = (t12049 + t12095 + t12119 + t12144) * t225;
    (t12143, t12147)
}
