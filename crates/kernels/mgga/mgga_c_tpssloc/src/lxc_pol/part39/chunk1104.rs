//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1104/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1104<F: Float>(t3523: F, t5005: F, t3572: F, t5019: F, t5024: F, t11147: F, t11778: F, t14165: F, t4582: F, t1735: F, t3252: F, t3578: F, t3248: F, t11642: F, t11644: F, t11649: F, t1174: F, t1227: F, t15434: F, t15438: F, t15446: F, t3518: F, t3527: F, t3531: F, t3577: F) -> (F,) {
    let t15448 = t5005 * t3523 / 3456.0;
    let t15450 = t5019 * t3572 / 432.0;
    let t15452 = t5024 * t3523 / 648.0;
    let t15453 = t11778 * t11147;
    let t15454 = t15453 * t14165;
    let t15455 = t4582 * t15454;
    let t15458 = t1735 * t3252;
    let t15459 = t3578 * t15458;
    let t15462 = t1735 * t3248;
    let t15463 = t3578 * t15462;
    let t15466 = t11642 / 4608.0 - t11644 / 6912.0 + t11649 - 7.0 / 648.0 * t1174 * t15434 - t15438 * t3518 / 3072.0 - t5005 * t3527 / 4608.0 - t5005 * t3531 / 2304.0 + t15446 - t15448 - t15450 + t15452 - 5.0 / 5184.0 * t1227 * t15455 - t3577 * t15459 / 4608.0 - t3577 * t15463 / 2304.0;
    (t15466,)
}
