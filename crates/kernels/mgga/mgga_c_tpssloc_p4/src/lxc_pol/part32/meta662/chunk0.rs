//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2093/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2093<F: Float>(t7303: F, t94490: F, t7291: F, t2122: F, t94319: F, t8034: F, t8003: F, t85660: F, t24574: F, t27412: F, t5052: F, t7299: F) -> (F, F, F, F, F, F, F) {
    let t94492 = F::cast_from(0.14621636149762012769e-1_f64) * t94490 * t7303;
    let t94494 = F::cast_from(0.14621636149762012769e-1_f64) * t94490 * t7291;
    let t94503 = t2122 * t94319;
    let t94514 = t8034 * t2122;
    let t94525 = t85660 * t8003;
    let t94535 = F::cast_from(0.10966227112321509577e-1_f64) * t24574 * t27412;
    let t94558 = t7299 * t5052;
    (t94492, t94494, t94503, t94514, t94525, t94535, t94558)
}
