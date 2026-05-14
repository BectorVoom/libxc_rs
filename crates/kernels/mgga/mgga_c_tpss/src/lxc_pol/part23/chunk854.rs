//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 854/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk854<F: Float>(t5697: F, t5699: F, t5701: F, t5984: F, t5986: F, t645: F, t1163: F, t118: F, t1273: F, t1865: F, t1897: F, t1899: F, t485: F, t544: F, t5519: F, t5521: F, t5524: F, t5534: F, t5707: F, t5712: F, t5756: F, t5759: F, t5991: F, t6054: F, t624: F, t626: F, t646: F) -> (F, F) {
    let t6058 = 2.0 * t5986 * t645 + t5697 + t5699 + t5701 + t5984;
    let t6061 = -t1163 * t1865 - t118 * t6054 + t1273 * t1899 - t1897 * t624 - t485 * t5984 + t544 * t6058 - 2.0 * t5986 * t646 - 2.0 * t5991 * t626 - t5519 - t5521 - t5524 - t5534 + t5707 + t5712 + t5756 - t5759;
    (t6058, t6061)
}
