//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1080/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1080<F: Float>(t105353: F, t105366: F, t105370: F, t105372: F, t105376: F, t105381: F, t87304: F, t87306: F, t87345: F, t98733: F, t98736: F, t98738: F, t98746: F, t98750: F, t98774: F, t98782: F, t98787: F, t98791: F, t98796: F, t98798: F) -> (F,) {
    let t108290 = -0.40372756094140390853e-3 * t105353 + 7.0 / 96.0 * t98733 - 35.0 / 36.0 * t87304 - 0.4069573814289351398e0 * t87306 + 7.0 / 384.0 * t98736 + 7.0 / 192.0 * t98738 + 0.84782787797694820791e-2 * t98746 - 0.24223653656484234512e-2 * t98750 - 0.84782787797694820791e-2 * t98774 - 0.40372756094140390854e-3 * t98782 + 0.20186378047070195427e-3 * t98787 + 0.20186378047070195427e-3 * t98791 - t105366 / 2.0 - 0.13565246047631171326e0 * t105370 - t105372 / 24.0 - 0.24223653656484234512e-2 * t105376 - 119.0 / 288.0 * t87345 - 7.0 / 192.0 * t98796 + 7.0 / 384.0 * t98798 - 5.0 / 64.0 * t105381;
    (t108290,)
}
