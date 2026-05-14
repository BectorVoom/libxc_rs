//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1289/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1289<F: Float>(t13298: F, t582: F, t10292: F, t1290: F, t21128: F, t619: F, t77: F, t1317: F, t6090: F, t21132: F, t1679: F, t4626: F, t1981: F, t4580: F, t615: F, t13447: F, t84: F) -> (F, F, F, F, F, F, F, F, F) {
    let t69165 = t13298 * t582;
    let t69186 = t10292 * t1290;
    let t69195 = t77 * t21128 * t619;
    let t69198 = t6090 * t1317;
    let t69203 = t77 * t21132 * t619;
    let t69206 = t1679 * t4626;
    let t69210 = t1981 * t4580;
    let t69228 = t77 * t615 * t4626;
    let t69232 = t77 * t84 * t13447;
    (t69165, t69186, t69195, t69198, t69203, t69206, t69210, t69228, t69232)
}
