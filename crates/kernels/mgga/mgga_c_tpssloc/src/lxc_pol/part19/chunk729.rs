//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 729/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk729<F: Float>(t1378: F, t3911: F, t1375: F, t1386: F, t3753: F, t3755: F, t3758: F, t3880: F, t3882: F, t3889: F, t568: F) -> (F, F) {
    let t3912 = t1378 * t3911;
    let t3914 = 2.0 * t1375 * t3889 - t1375 * t3912 - 2.0 * t1386 * t3758 - 2.0 * t1386 * t3882 + t3753 * t568 + 2.0 * t3755 * t568 + t3880 * t568;
    (t3912, t3914)
}
