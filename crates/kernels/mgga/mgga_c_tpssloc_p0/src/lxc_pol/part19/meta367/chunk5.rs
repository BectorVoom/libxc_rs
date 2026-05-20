//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1349/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1349<F: Float>(t42790: F, t42824: F, t42860: F, t42899: F, t42933: F, t42966: F, t43034: F, t43079: F, t225: F, t10427: F, t13969: F, t3130: F) -> (F, F, F) {
    let t43082 = t42790 + t42824 + t42860 + t42899 + t42933 + t42966 + t43034 + t43079;
    let t43083 = t43082 * t225;
    let t43094 = t3130 * t13969 * t10427;
    (t43082, t43083, t43094)
}
