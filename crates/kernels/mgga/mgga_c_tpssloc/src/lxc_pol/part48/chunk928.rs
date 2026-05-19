//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 928/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk928<F: Float>(t1307: F, t1352: F, t2006: F, t22633: F, t6976: F, t22751: F, t31195: F, t22892: F, t22893: F, t31194: F, t22642: F, t22690: F, t31193: F) -> (F, F, F, F) {
    let t114056 = F::cast_from(0.6579736267392905746e-1_f64) * t22633 * t6976 * t2006 * t1307 * t1352;
    let t114057 = t22751 * t31195;
    let t114058 = F::cast_from(0.15352717957250113407e0_f64) * t114057;
    let t114060 = t22892 * t22893 * t31194;
    let t114061 = F::cast_from(0.3289868133696452873e-1_f64) * t114060;
    let t114064 = F::cast_from(0.16449340668482264365e-1_f64) * t22642 * t22690 * t31193;
    (t114056, t114058, t114061, t114064)
}
