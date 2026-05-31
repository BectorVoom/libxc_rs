//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1487/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1487<F: Float>(t11539: F, t4724: F, t1174: F, t15239: F, t475: F, t1214: F, t248: F, t3494: F, t4977: F, t4582: F, t3516: F, t12652: F, t4987: F) -> (F, F, F, F, F) {
    let t15522 = t11539 * t4724;
    let t15524 = t1174 * t15522 / F::cast_from(324.0_f64);
    let t15525 = t15239 * t475;
    let t15527 = t248 * t1214 * t15525;
    let t15530 = t4977 * t3494;
    let t15531 = t4582 * t15530;
    let t15534 = t4977 * t3516;
    let t15535 = t4582 * t15534;
    let t15540 = t4987 * t12652;
    (t15524, t15527, t15531, t15535, t15540)
}
