//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1194/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1194<F: Float>(t3969: F, t5620: F, t1471: F, t18069: F, t18083: F, t18086: F, t18098: F, t18104: F, t18119: F, t18122: F, t3952: F, t3956: F, t3963: F, t3974: F, t3979: F, t3983: F, t5610: F) -> (F, F) {
    let t19880 = t5620 * t3969;
    let t19888 = t5610 * t3952 / 1536.0 - t18098 * t3956 / 1536.0 + t18104 / 2304.0 - t18119 / 432.0 - t18122 + t18086 / 3456.0 + t18069 * t3963 / 2304.0 - t18083 * t1471 / 432.0 + t19880 / 3456.0 + 5.0 / 6912.0 * t5620 * t3974 - t5620 * t3979 / 1152.0 + t5620 * t3983 / 2304.0;
    (t19880, t19888)
}
