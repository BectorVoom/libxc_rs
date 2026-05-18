//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 971/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk971<F: Float>(t22759: F, t6388: F, t6936: F, t1985: F, t214: F, t225: F, t28107: F, t567: F, t120308: F, t120544: F, t7700: F, t120532: F) -> (F, F, F, F, F) {
    let t127299 = t6936 * t22759 * t6388;
    let t127316 = F::new(0.16449340668482264365e-1) * t1985 * t214 * t28107 * t225 * t567;
    let t127325 = F::new(0.3289868133696452873e-1) * t120308;
    let t127328 = F::new(0.3289868133696452873e-1) * t1985 * t120544 * t7700;
    let t127346 = F::new(0.76763589786250567036e-1) * t120532;
    (t127299, t127316, t127325, t127328, t127346)
}
