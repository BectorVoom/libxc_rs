//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 634/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk634<F: Float>(t2835: F, t2836: F, t2843: F, t2848: F, t2852: F, t408: F, t1019: F, t1023: F, t1044: F, t1022: F, t404: F, t394: F) -> (F, F, F, F, F, F) {
    let t2854 = t2835 - F::new(0.11872222222222222222e-1) * t2836 - F::new(0.11872222222222222222e-1) * t2843 + F::new(0.35616666666666666666e-1) * t2848 + F::new(0.17808333333333333333e-1) * t2852;
    let t2856 = F::new(0.621814e-1) * t2854 * t408;
    let t2857 = t1019 * t1023;
    let t2859 = F::new(2.0) * t2857 * t1044;
    let t2860 = t1022 * t404;
    let t2861 = F::new(1.0) / t2860;
    let t2862 = t394 * t2861;
    (t2854, t2856, t2857, t2859, t2861, t2862)
}
