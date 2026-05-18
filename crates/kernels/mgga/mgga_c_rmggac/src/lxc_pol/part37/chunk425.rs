//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 425/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk425<F: Float>(t7599: F, t8743: F, t27: F, t3839: F, t1635: F, t649: F, t3826: F, t1624: F, t1627: F, t7603: F, t8729: F, t8731: F, t8733: F, t8735: F, t8737: F, t8739: F, t8741: F) -> (F, F, F, F, F, F, F, F) {
    let t8744 = t7599 * t8743;
    let t8746 = t3839 * t27;
    let t8747 = t649 * t1635;
    let t8748 = t8746 * t8747;
    let t8750 = t3826 * t27;
    let t8751 = t649 * t1624;
    let t8752 = t8750 * t8751;
    let t8754 = t649 * t1627;
    let t8755 = t7603 * t8754;
    let t8757 = F::new(0.14967802127329760705e-1) * t8729 - F::new(0.99785347515531738034e-2) * t8731 - F::new(0.99785347515531738034e-2) * t8733 + F::new(0.88507694033737208925e-3) * t8735 - F::new(0.10620923284048465071e-2) * t8737 - F::new(0.39914139006212695213e-1) * t8739 + F::new(0.26609426004141796809e-1) * t8741 - F::new(0.13637330827122670865e-1) * t8744 + F::new(0.22728884711871118108e-1) * t8748 + F::new(0.45360193192290319575e-3) * t8752 - F::new(0.63504270469206447405e-3) * t8755;
    (t8744, t8747, t8748, t8751, t8752, t8754, t8755, t8757)
}
