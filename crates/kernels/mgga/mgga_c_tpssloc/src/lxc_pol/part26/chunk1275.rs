//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1275/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1275<F: Float>(t23005: F, t6579: F, t2631: F, t852: F, t1888: F, t232: F, t6646: F, t23181: F, t2710: F, t828: F, t22996: F, t2632: F) -> (F, F, F, F, F) {
    let t81697 = t6579 * t23005;
    let t81699 = t852 * t2631;
    let t81702 = t1888 * t6646 * t81699 * t232;
    let t81704 = t6579 * t23181;
    let t81709 = t1888 * t6646 * t2710 * t828 * t232;
    let t81713 = t1888 * t22996 * t81699 * t2632;
    (t81697, t81702, t81704, t81709, t81713)
}
