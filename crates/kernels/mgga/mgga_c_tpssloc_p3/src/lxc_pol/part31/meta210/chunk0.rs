//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 933/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk933<F: Float>(t2718: F, t5636: F, t2728: F, t5585: F, t1510: F, t4295: F, t5612: F, t860: F, t5617: F, t235: F, t5631: F, t1499: F, t1523: F, t1525: F, t226: F, t255: F, t4166: F, t5575: F, t812: F) -> (F, F, F, F, F, F, F) {
    let t5637 = t2718 * t5636;
    let t5645 = t2728 * t5585;
    let t5648 = t4295 * t1510;
    let t5651 = t860 * t5612;
    let t5653 = t860 * t5617;
    let t5655 = t235 * t5631;
    let t5657 = F::new(2.0) * t1499 * t1525 - F::new(2.0) * t1523 * t4166 + t226 * t5655 + t255 * t5575 + F::new(2.0) * t5645 * t812 - F::new(2.0) * t5648 * t812 - t5651 * t812 - t5653 * t812;
    (t5637, t5645, t5648, t5651, t5653, t5655, t5657)
}
