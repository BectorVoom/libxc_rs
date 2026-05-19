//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 735/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk735<F: Float>(t22940: F, t22870: F, t539: F, t12033: F, t1375: F, t2016: F, t22688: F, t22905: F, t22908: F, t22910: F, t22913: F, t22918: F, t22922: F, t22924: F, t22926: F, t22928: F, t22931: F, t22936: F, t3758: F, t3889: F, t568: F, t6958: F, t6963: F, t6993: F) -> (F, F) {
    let t22941 = F::cast_from(0.38381794893125283518e-1_f64) * t22940;
    let t22942 = t539 * t22870;
    let t22946 = F::new(2.0) * t6958 * t3889 + F::cast_from(0.49348022005446793095e-1_f64) * t22688 - t1375 * t22905 + t22908 + t22910 - t12033 * t2016 + F::new(2.0) * t1375 * t22913 - F::cast_from(0.16449340668482264365e-1_f64) * t22918 + t22922 + t22924 + t22926 - F::cast_from(0.82246703342411321824e-2_f64) * t22928 - F::cast_from(0.3289868133696452873e-1_f64) * t22931 + F::cast_from(0.16449340668482264365e-1_f64) * t22936 + F::new(4.0) * t3758 * t6963 - t22941 + t22942 * t568 - F::new(2.0) * t3758 * t6993;
    (t22942, t22946)
}
