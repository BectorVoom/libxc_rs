//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1266/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1266<F: Float>(t22986: F, t22997: F, t2647: F, t6646: F, t1887: F, t23069: F, t22989: F, t22690: F, t23153: F, t23171: F, t6561: F, t80741: F) -> (F, F, F, F, F) {
    let t81589 = t22986 * t6646 * t22997 * t2647;
    let t81591 = t23069 * t1887;
    let t81592 = t81591 * t22989;
    let t81595 = t23171 * t22690 * t23153;
    let t81597 = t80741 * t6561;
    (t81589, t81591, t81592, t81595, t81597)
}
