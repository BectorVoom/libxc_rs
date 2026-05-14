//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1116/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1116<F: Float>(t19456: F, t8675: F, t31908: F, t4028: F, t33726: F, t4034: F, t652: F, t7408: F, t7467: F, t24932: F, t7468: F, t27888: F, t26003: F, t7266: F, t1874: F, t96238: F) -> (F, F, F, F, F, F, F, F) {
    let t123122 = t19456 * t8675;
    let t123124 = t4028 * t31908;
    let t123126 = t4034 * t33726;
    let t123129 = t652 * t7408 * t7467;
    let t123138 = t24932 * t7468;
    let t123140 = t27888 * t7468;
    let t123142 = t7266 * t26003;
    let t123155 = t96238 * t1874;
    (t123122, t123124, t123126, t123129, t123138, t123140, t123142, t123155)
}
