//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 696/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk696<F: Float>(t5: F, t22557: F, t112: F, t1266: F, t6534: F, t652: F, t192: F, t532: F, t1982: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t22558 = piecewise3::<F>(t8, F::new(0.0), t22557);
    let t22559 = t22558 * t112;
    let t22561 = t1266 * t6534;
    let t22563 = F::new(4.0) * t652 * t22561;
    let t22573 = t192 * t532;
    let t22574 = t1982 * t22573;
    (t22558, t22559, t22561, t22563, t22573, t22574)
}
