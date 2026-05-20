//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 979/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk979<F: Float>(t33185: F, t33659: F, t24465: F, t28896: F, t28899: F, t16524: F, t2039: F, t28017: F, t3941: F, t33656: F, t7769: F, t94170: F) -> (F, F, F, F, F, F, F) {
    let t127673 = F::new(54.0) * t33185 * t33659;
    let t127677 = F::new(54.0) * t24465 * t28896;
    let t127679 = F::new(27.0) * t24465 * t28899;
    let t127681 = F::new(54.0) * t16524 * t33659;
    let t127684 = F::new(27.0) * t3941 * t2039 * t28017;
    let t127686 = F::new(54.0) * t16524 * t33656;
    let t127688 = F::new(54.0) * t94170 * t7769;
    (t127673, t127677, t127679, t127681, t127684, t127686, t127688)
}
