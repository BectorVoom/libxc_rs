//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 850/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk850<F: Float>(t5640: F, t5642: F, t1731: F, t347: F, t5623: F, t1730: F, t1733: F, t373: F, t5624: F, t5626: F, t5629: F, t5631: F, t5634: F, t5639: F, t991: F, t1735: F, t2814: F) -> (F, F, F, F) {
    let t5643 = t5640 * t5642;
    let t5646 = t1731 * t347 * t5623;
    let t5648 = -t1730 * t5646 - t1733 * t5629 + t373 * t5624 - t5626 * t991 + 2.0 * t5631 * t5634 - t5639 * t5643;
    let t5652 = t1735 * t2814;
    (t5643, t5646, t5648, t5652)
}
