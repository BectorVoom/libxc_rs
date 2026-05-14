//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1167/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1167<F: Float>(t2689: F, t5605: F, t5614: F, t962: F, t1723: F, t2650: F, t1721: F, t2655: F, t339: F, t18104: F, t18107: F, t18110: F, t2646: F, t2672: F, t2693: F, t2700: F, t363: F, t5620: F, t931: F, t951: F) -> (F, F, F, F, F) {
    let t18113 = t5605 * t2689;
    let t18119 = t5614 * t962;
    let t18122 = t1723 * t2650 / 6912.0;
    let t18126 = t339 * t1721 * t2655;
    let t18129 = -t5620 * t2672 / 1152.0 + t18104 / 1152.0 - t18107 * t951 / 144.0 - t18110 * t931 / 54.0 + t18113 / 432.0 + t5605 * t2693 / 288.0 + t5605 * t2700 / 216.0 - t18119 / 216.0 - t18122 - t5605 * t2646 / 144.0 + 19.0 / 864.0 * t18126 * t363;
    (t18113, t18119, t18122, t18126, t18129)
}
