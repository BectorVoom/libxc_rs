//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2031/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2031<F: Float>(t2121: F, t3427: F, t7295: F, t11947: F, t7394: F, t2157: F, t43706: F, t1453: F, t81439: F, t26129: F, t81442: F, t22470: F, t4067: F) -> (F, F, F, F, F, F) {
    let t86501 = t2121 * t3427 * t7295;
    let t86517 = t7394 * t11947;
    let t86524 = t2157 * t43706;
    let t86586 = t81439 * t1453;
    let t86588 = t81442 * t26129;
    let t86589 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t86588;
    let t86590 = t22470 * t4067;
    (t86501, t86517, t86524, t86586, t86589, t86590)
}
