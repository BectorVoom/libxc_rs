//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1076/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1076<F: Float>(t84873: F, t84894: F, t84916: F, t84937: F, t24234: F, t814: F, t10016: F, t2051: F, t226: F, t235: F, t4291: F, t812: F, t81563: F, t81568: F, t81571: F, t81575: F, t81585: F, t81589: F, t81592: F, t81595: F, t81600: F, t81602: F, t81606: F, t81610: F, t81615: F, t829: F, t84842: F, t84851: F) -> (F, F) {
    let t84939 = t84873 + t84894 + t84916 + t84937;
    let t84945 = t814 * t24234;
    let t84949 = -0.19739208802178717238e0 * t81563 + 0.9869604401089358619e-1 * t81568 - 0.24674011002723396548e-1 * t81571 + 0.9869604401089358619e-1 * t81575 - 3.0 * t4291 * t84842 * t829 + t10016 * t2051 - 0.29608813203268075857e0 * t81585 + 0.9869604401089358619e-1 * t81589 - 0.46058153871750340221e0 * t81592 - 0.49348022005446793095e-1 * t81595 - t84851 + 0.15626873635058151147e0 * t81600 + 0.38381794893125283518e0 * t81602 + t226 * t235 * t84939 + 0.19739208802178717238e0 * t81606 + 0.9869604401089358619e-1 * t81610 + 0.49348022005446793095e-1 * t81615 - 3.0 * t812 * t84945 * t829;
    (t84939, t84949)
}
