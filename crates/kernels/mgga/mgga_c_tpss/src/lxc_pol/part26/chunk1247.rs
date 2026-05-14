//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1247/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1247<F: Float>(t22124: F, t22197: F, t3: F, t1670: F, t1904: F, t21555: F, t21557: F, t21559: F, t21562: F, t21565: F, t21568: F, t21571: F, t5474: F, t5477: F, t548: F, t6552: F) -> (F, F, F, F) {
    let t22198 = t22124 + t22197;
    let t22199 = t3 * t22198;
    let t22209 = param_d * t22198;
    let t22217 = 6.0 * t1670 * t6552 + 6.0 * t1904 * t5474 + 3.0 * t1904 * t5477 + t22209 * t548 + t21555 + t21557 + t21559 + t21562 + t21565 + t21568 + t21571;
    (t22198, t22199, t22209, t22217)
}
