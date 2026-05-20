//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1039/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1039<F: Float>(t1873: F, t91854: F, t23938: F, t6534: F, t91857: F, t26977: F, t22479: F, t7042: F, t2319: F, t8518: F, t2307: F, t8513: F, t8514: F) -> (F, F, F, F, F, F, F) {
    let t115813 = F::new(4.0) * t91854 * t1873;
    let t115815 = F::new(4.0) * t23938 * t6534;
    let t115817 = F::new(2.0) * t91857 * t1873;
    let t115819 = F::new(4.0) * t26977 * t6534;
    let t115821 = F::new(2.0) * t7042 * t22479;
    let t115824 = t8518 * t2319;
    let t115829 = t8513 * t8514 * t2307;
    (t115813, t115815, t115817, t115819, t115821, t115824, t115829)
}
