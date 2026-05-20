//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2144/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2144<F: Float>(t10868: F, t820: F, t3070: F, t3072: F, t10489: F, t3117: F, t1015: F, t10472: F, t42559: F, t10870: F, t3048: F, t204: F, t376: F) -> (F, F, F, F, F, F) {
    let t43198 = t820 * t10868;
    let t43200 = t3070 * t43198 * t3072;
    let t43206 = t3117 * t10489;
    let t43211 = t10472 * t1015 * t42559;
    let t43214 = t3048 * t10870;
    let t43216 = t204 * t376;
    (t43198, t43200, t43206, t43211, t43214, t43216)
}
