//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 945/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk945<F: Float>(t1985: F, t22662: F, t31137: F, t22716: F, t8459: F, t31170: F, t3809: F, t22779: F, t31162: F, t22759: F, t3793: F, t6936: F) -> (F, F, F, F, F) {
    let t113961 = F::new(0.16449340668482264365e-1) * t1985 * t31137 * t22662;
    let t113963 = F::new(0.12793931631041761173e0) * t22716 * t8459;
    let t113964 = t31170 * t3809;
    let t113966 = t22779 * t31162;
    let t113969 = t6936 * t22759 * t3793;
    (t113961, t113963, t113964, t113966, t113969)
}
