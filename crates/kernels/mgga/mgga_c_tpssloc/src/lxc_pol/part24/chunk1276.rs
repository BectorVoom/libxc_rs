//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1276/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1276<F: Float>(t1372: F, t1992: F, t3850: F, t550: F, t6976: F, t3791: F, t22700: F, t6914: F, t3787: F, t6955: F, t22699: F, t22704: F, t22705: F) -> (F, F, F, F, F, F) {
    let t81092 = t1992 * t6976 * t1372 * t3850 * t550;
    let t81094 = t1372 * t3791;
    let t81097 = t1992 * t6976 * t81094 * t550;
    let t81099 = t6914 * t22700;
    let t81105 = t3787 * t6955;
    let t81115 = t22704 * t22705 * t22699;
    (t81092, t81094, t81097, t81099, t81105, t81115)
}
