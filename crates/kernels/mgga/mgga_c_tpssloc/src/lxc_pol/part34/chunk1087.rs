//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1087/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1087<F: Float>(t105519: F, t105698: F, t1492: F, t2054: F, t21053: F, t25168: F, t259: F, t26728: F, t29040: F, t67305: F, t67339: F, t87898: F, t87915: F, t99003: F, t99022: F, t99036: F) -> (F,) {
    let t108448 = -3.0 * t67305 * t2054 + 0.11514538467937585055e0 * t99003 + 0.19739208802178717238e0 * t105519 - 3.0 * t67339 * t2054 - 0.29608813203268075857e0 * t105698 - 0.24674011002723396548e-1 * t99022 - 0.15626873635058151147e0 * t87898 - 0.49348022005446793095e-1 * t87915 + 3.0 * t1492 * t29040 * t259 + 0.9869604401089358619e-1 * t99036 - 18.0 * t25168 * t26728 * t21053;
    (t108448,)
}
