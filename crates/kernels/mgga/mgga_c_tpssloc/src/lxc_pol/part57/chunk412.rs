//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 412/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk412<F: Float>(t40: F, t52: F, t510: F, t5493: F, t4100: F, t4102: F, t185: F, t5392: F, t2658: F, t1484: F, t4310: F, t1462: F, t4205: F, t2433: F, t5398: F, t73: F, t2440: F, t76: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t5494 = t510 * t5493;
    let t5497 = 2.0 * t4100;
    let t5498 = 8.0 * t4102;
    let t5499 = t185 * t5392;
    let t5501 = 12.0 * t2658 * t5499;
    let t5502 = t4310 * t1484;
    let t5506 = 8.0 * t4205 * t1462;
    let t5512 = piecewise3(t146, 0.0, 4.0 / 9.0 * t2433 * t5392 + 4.0 / 3.0 * t73 * t5398);
    let t5518 = piecewise3(t150, 0.0, 4.0 / 9.0 * t2440 * t5392 - 4.0 / 3.0 * t76 * t5398);
    (t5494, t5497, t5498, t5501, t5502, t5506, t5512, t5518)
}
