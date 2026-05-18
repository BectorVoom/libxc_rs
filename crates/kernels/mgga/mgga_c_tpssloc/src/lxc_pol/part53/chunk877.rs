//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 877/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk877<F: Float>(t31645: F, t6888: F, t6883: F, t8622: F, t22666: F, t8621: F, t1985: F, t8612: F, t225: F, t8729: F, t31320: F, t798: F, t8728: F) -> (F, F, F, F, F, F, F, F) {
    let t31646 = t6888 * t31645;
    let t31648 = t6883 * t8622;
    let t31650 = t22666 * t8621;
    let t31651 = t1985 * t31650;
    let t31662 = t6883 * t8612;
    let t31964 = t8729 * t225;
    let t31971 = F::new(0.16449340668482264365e-1) * t31320;
    let t31974 = t798 * t8728;
    (t31646, t31648, t31650, t31651, t31662, t31964, t31971, t31974)
}
