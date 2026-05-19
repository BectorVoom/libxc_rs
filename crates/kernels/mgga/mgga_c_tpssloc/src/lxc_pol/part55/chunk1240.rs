//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1240/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1240<F: Float>(t22633: F, t26403: F, t3807: F, t6976: F, t114057: F, t114060: F, t22751: F, t32741: F, t1338: F, t32726: F, t114069: F, t1799: F, t6637: F, t6888: F) -> (F, F, F, F, F, F) {
    let t120467 = F::cast_from(0.3289868133696452873e-1_f64) * t22633 * t6976 * t26403 * t3807;
    let t120468 = F::cast_from(0.76763589786250567036e-1_f64) * t114057;
    let t120469 = F::cast_from(0.16449340668482264365e-1_f64) * t114060;
    let t120470 = t22751 * t32741;
    let t120471 = F::cast_from(0.76763589786250567037e-1_f64) * t120470;
    let t120475 = t1338 * t32726;
    let t120483 = F::cast_from(0.3289868133696452873e-1_f64) * t6888 * t6637 * t114069 * t1799;
    (t120467, t120468, t120469, t120471, t120475, t120483)
}
