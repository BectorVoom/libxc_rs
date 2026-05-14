//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1163/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1163<F: Float>(t22882: F, t22892: F, t22893: F, t12156: F, t6637: F, t6968: F, t80732: F, t1372: F, t1992: F, t3850: F, t550: F, t6976: F, t3791: F, t22700: F, t6914: F, t3787: F, t6955: F) -> (F, F, F, F, F, F, F) {
    let t81083 = t22892 * t22893 * t22882;
    let t81087 = t80732 * t6637 * t6968 * t12156;
    let t81092 = t1992 * t6976 * t1372 * t3850 * t550;
    let t81094 = t1372 * t3791;
    let t81097 = t1992 * t6976 * t81094 * t550;
    let t81099 = t6914 * t22700;
    let t81105 = t3787 * t6955;
    (t81083, t81087, t81092, t81094, t81097, t81099, t81105)
}
