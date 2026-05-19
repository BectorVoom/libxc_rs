//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1328/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1328<F: Float>(t1868: F, t4072: F, t24994: F, t8449: F, t22751: F, t32731: F, t22633: F, t22635: F, t31099: F, t5187: F, t31090: F, t97721: F) -> (F, F, F, F, F) {
    let t120148 = t1868 * t4072;
    let t120172 = t8449 * t24994;
    let t120179 = t22751 * t32731;
    let t120180 = F::cast_from(0.76763589786250567037e-1_f64) * t120179;
    let t120184 = F::cast_from(0.3289868133696452873e-1_f64) * t22633 * t22635 * t31099 * t5187;
    let t120196 = F::cast_from(0.6579736267392905746e-1_f64) * t22633 * t22635 * t31090 * t97721;
    (t120148, t120172, t120180, t120184, t120196)
}
