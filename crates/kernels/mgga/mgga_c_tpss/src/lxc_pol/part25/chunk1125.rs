//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1125/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1125<F: Float>(t4192: F, t4202: F, t1082: F, t5198: F, t1089: F, t12210: F, t4205: F, t4238: F, t4258: F, t242: F, t3060: F, t5249: F) -> (F, F, F, F, F) {
    let t15478 = F::new(0.11696447245269292414e1) * t4192 * t4202;
    let t15479 = t5198 * t1082;
    let t15481 = F::new(0.35089341735807877242e1) * t1089 * t15479;
    let t15482 = t4205 * t12210;
    let t15484 = F::new(0.34631718211362927518e2) * t1089 * t15482;
    let t15485 = t4258 * t4238;
    let t15488 = t242 * t3060 * t5249;
    (t15478, t15481, t15484, t15485, t15488)
}
