//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1138/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1138<F: Float>(t1888: F, t232: F, t2710: F, t6646: F, t828: F, t22996: F, t2632: F, t81699: F, t2587: F, t81151: F, t23172: F, t25084: F, t9634: F) -> (F, F, F, F, F) {
    let t81709 = t1888 * t6646 * t2710 * t828 * t232;
    let t81713 = t1888 * t22996 * t81699 * t2632;
    let t81715 = t81151 * t2587;
    let t81716 = t81715 * t23172;
    let t81724 = t25084 * t9634;
    (t81709, t81713, t81715, t81716, t81724)
}
