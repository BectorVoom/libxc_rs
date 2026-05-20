//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1104/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1104<F: Float>(t591: F, t9688: F, t2386: F, t240: F, t2385: F, t2558: F, t686: F, t685: F, t120: F, t118: F, t123: F, t116: F, t268: F, t8705: F) -> (F, F, F, F, F, F, F, F) {
    let t39275 = t9688 * t591;
    let t39277 = t2386 * t240;
    let t39278 = t2385 * t39277;
    let t39280 = t686 * t2558;
    let t39281 = t685 * t39280;
    let t39283 = t120 * t2558;
    let t39284 = t118 * t39283;
    let t39286 = F::powf(t123, -F::new(0.25e1));
    let t39289 = t39286 * t116 * t8705 * t268;
    (t39275, t39277, t39278, t39280, t39281, t39283, t39284, t39289)
}
