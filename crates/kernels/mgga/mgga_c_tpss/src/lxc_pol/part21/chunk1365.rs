//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1365/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1365<F: Float>(t1789: F, t4543: F, t1665: F, t5779: F, t1278: F, t1284: F, t13292: F, t1666: F, t1673: F, t1782: F, t18563: F, t18602: F, t20083: F, t3399: F, t3413: F, t4544: F, t4562: F, t5761: F, t60626: F, t62165: F, t6280: F, t6296: F, t66099: F, t66141: F) -> (F,) {
    let t66149 = 2.0 * t4543 * t1789;
    let t66155 = 2.0 * t1665 * t5779;
    let t66157 = 2.0 * t5761 * t4562 + t6280 * t3413 + t1278 * (t66099 + t66141) + t1782 * t13292 + t18563 * t1673 + t3399 * t6296 + t1666 * t18602 + t66149 + t62165 + 2.0 * t4544 * t5779 + 2.0 * t20083 * t1284 + t66155 + 2.0 * t60626;
    (t66157,)
}
