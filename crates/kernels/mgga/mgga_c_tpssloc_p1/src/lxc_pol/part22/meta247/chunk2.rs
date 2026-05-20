//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1358/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1358<F: Float>(t2843: F, t290: F, t2860: F, t919: F, t2904: F, t938: F, t10629: F, t315: F) -> (F, F, F, F) {
    let t10704 = F::new(1.0) / t2843 / t290;
    let t10740 = t919 * t2860;
    let t10747 = t938 * t2904;
    let t10756 = t315 * t10629;
    (t10704, t10740, t10747, t10756)
}
