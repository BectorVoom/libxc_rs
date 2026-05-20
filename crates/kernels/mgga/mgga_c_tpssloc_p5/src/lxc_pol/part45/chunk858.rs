//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 858/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk858<F: Float>(t1985: F, t31120: F, t6906: F, t6992: F, t6889: F, t22674: F, t8458: F, t6897: F, t2006: F, t214: F) -> (F, F, F, F, F, F, F) {
    let t31122 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t31120;
    let t31123 = t6906 * t6992;
    let t31124 = t6889 * t31123;
    let t31126 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t31124;
    let t31127 = t22674 * t8458;
    let t31129 = F::cast_from(0.82246703342411321825e-2_f64) * t6897 * t31127;
    let t31137 = t214 * t2006;
    (t31122, t31123, t31124, t31126, t31127, t31129, t31137)
}
