//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 925/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk925<F: Float>(t1999: F, t22866: F, t1338: F, t6955: F, t1372: F, t552: F, t117: F, t547: F, t67: F, t6559: F) -> (F, F, F, F, F) {
    let t22867 = t22866 * t1999;
    let t22868 = F::new(0.11304371706359309439e-1) * t22867;
    let t22873 = t1338 * t6955;
    let t22881 = t552 * t1372;
    let t22891 = t547 * t67 * t117;
    let t22892 = t6559 * t22891;
    (t22867, t22868, t22873, t22881, t22892)
}
