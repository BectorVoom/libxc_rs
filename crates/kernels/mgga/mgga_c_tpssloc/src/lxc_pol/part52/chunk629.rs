//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 629/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk629<F: Float>(t1044: F, t248: F, t4347: F, t1009: F, t1603: F, t1011: F, t1019: F, t1040: F, t1611: F, t4353: F, t4356: F, t4358: F, t4361: F, t4398: F, t4402: F, t4480: F, t4482: F, t4485: F, t4487: F, t4491: F, t4495: F, t4500: F) -> (F, F, F, F, F, F) {
    let t4636 = t248 * t1044 * t4347;
    let t4639 = t1603 * t1009;
    let t4640 = t4639 * t1011;
    let t4641 = t4640 * t1019;
    let t4644 = t1611 * t1040;
    let t4649 = -t4353 + t4356 + t4358 - t4361 + t4398 + t4402 + t4480 + t4482 - t4485 - t4487 + t4491 - t4495 - t4500;
    (t4636, t4639, t4640, t4641, t4644, t4649)
}
