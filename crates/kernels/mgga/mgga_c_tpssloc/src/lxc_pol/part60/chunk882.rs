//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 882/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk882<F: Float>(t5493: F, t576: F, t33191: F, t8657: F, t33185: F, t33656: F, t33659: F, t24465: F, t28896: F, t28899: F, t16524: F, t2039: F, t28017: F, t3941: F, t7769: F, t94170: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t127643 = t576 * t5493;
    let t127646 = 27.0 * t33191;
    let t127669 = 27.0 * t127643 * t8657;
    let t127671 = 54.0 * t33185 * t33656;
    let t127673 = 54.0 * t33185 * t33659;
    let t127677 = 54.0 * t24465 * t28896;
    let t127679 = 27.0 * t24465 * t28899;
    let t127681 = 54.0 * t16524 * t33659;
    let t127684 = 27.0 * t3941 * t2039 * t28017;
    let t127686 = 54.0 * t16524 * t33656;
    let t127688 = 54.0 * t94170 * t7769;
    (t127646, t127669, t127671, t127673, t127677, t127679, t127681, t127684, t127686, t127688)
}
