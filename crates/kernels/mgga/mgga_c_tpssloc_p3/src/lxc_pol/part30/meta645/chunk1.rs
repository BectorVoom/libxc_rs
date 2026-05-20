//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2058/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2058<F: Float>(t13783: F, t1920: F, t4338: F, t14192: F, t6717: F, t13965: F, t6755: F, t25577: F, t3103: F, t1933: F, t23479: F, t88405: F) -> (F, F, F, F, F) {
    let t88625 = t1920 * t13783 * t4338 / F::new(324.0);
    let t88636 = t6717 * t14192 / F::new(432.0);
    let t88645 = t6755 * t13965;
    let t88648 = t25577 * t3103 / F::new(1152.0);
    let t88689 = F::cast_from(0.20186378047070195428e-3_f64) * t1933 * t88405 * t23479;
    (t88625, t88636, t88645, t88648, t88689)
}
