//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1182/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1182<F: Float>(t20853: F, t6605: F, t815: F, t20944: F, t81959: F, t1894: F, t20756: F, t236: F, t81969: F, t20994: F, t6581: F, t20800: F, t6591: F, t20974: F, t23146: F, t87304: F, t87306: F, t87345: F, t98733: F, t98736: F, t98738: F, t98746: F, t98750: F, t98774: F, t98782: F, t98787: F, t98791: F, t98796: F, t98798: F) -> (F,) {
    let t105353 = t6605 * t815 * t20853;
    let t105366 = t81959 * t20944;
    let t105370 = t81969 * t1894 * t236 * t20756;
    let t105372 = t6581 * t20994;
    let t105376 = t6591 * t1894 * t236 * t20800;
    let t105381 = t23146 * t20974;
    let t105383 = -0.20186378047070195427e-3 * t105353 + 7.0 / 192.0 * t98733 - 35.0 / 72.0 * t87304 - 0.2034786907144675699e0 * t87306 + 7.0 / 768.0 * t98736 + 7.0 / 384.0 * t98738 + 0.42391393898847410397e-2 * t98746 - 0.12111826828242117256e-2 * t98750 - 0.42391393898847410397e-2 * t98774 - 0.20186378047070195427e-3 * t98782 + 0.10093189023535097714e-3 * t98787 + 0.10093189023535097714e-3 * t98791 - t105366 / 4.0 - 0.67826230238155856634e-1 * t105370 - t105372 / 48.0 - 0.12111826828242117256e-2 * t105376 - 119.0 / 576.0 * t87345 - 7.0 / 384.0 * t98796 + 7.0 / 768.0 * t98798 - 5.0 / 128.0 * t105381;
    (t105383,)
}
