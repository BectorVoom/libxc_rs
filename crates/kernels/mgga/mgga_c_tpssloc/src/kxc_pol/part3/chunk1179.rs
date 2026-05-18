//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1179/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1179<F: Float>(t1735: F, t3252: F, t3578: F, t3248: F, t11642: F, t11644: F, t11649: F, t1174: F, t1227: F, t15434: F, t15438: F, t15446: F, t15448: F, t15450: F, t15452: F, t15455: F, t3518: F, t3527: F, t3531: F, t3577: F, t5005: F) -> F {
    let t15458 = t1735 * t3252;
    let t15459 = t3578 * t15458;
    let t15462 = t1735 * t3248;
    let t15463 = t3578 * t15462;
    let t15466 = t11642 / F::new(4608.0) - t11644 / F::new(6912.0) + t11649 - F::new(7.0) / F::new(648.0) * t1174 * t15434 - t15438 * t3518 / F::new(3072.0) - t5005 * t3527 / F::new(4608.0) - t5005 * t3531 / F::new(2304.0) + t15446 - t15448 - t15450 + t15452 - F::new(5.0) / F::new(5184.0) * t1227 * t15455 - t3577 * t15459 / F::new(4608.0) - t3577 * t15463 / F::new(2304.0);
    t15466
}
