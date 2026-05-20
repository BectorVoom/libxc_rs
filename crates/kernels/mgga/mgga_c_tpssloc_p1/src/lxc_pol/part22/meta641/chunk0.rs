//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2181/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2181<F: Float>(t19844: F, t3726: F, t1831: F, t53906: F, t16336: F, t5314: F, t53880: F, t19930: F, t3866: F, t1351: F, t6414: F, t120: F, t19731: F) -> (F, F, F, F, F, F, F) {
    let t56738 = t3726 * t19844;
    let t56776 = t53906 * t1831;
    let t56779 = t16336 * t5314;
    let t56795 = t53880 * t1831;
    let t56797 = t3866 * t19930;
    let t56812 = t6414 * t1351;
    let t56817 = t120 * t19731;
    (t56738, t56776, t56779, t56795, t56797, t56812, t56817)
}
