//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1255/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1255<F: Float>(t81398: F, t12438: F, t12444: F, t22653: F, t22656: F, t22905: F, t3758: F, t3882: F, t3889: F, t539: F, t568: F, t6958: F, t6993: F, t81011: F, t81379: F, t81386: F, t81393: F, t81395: F) -> F {
    let t81399 = F::cast_from(0.13707783890401886971e-2_f64) * t81398;
    let t81404 = -F::cast_from(0.24674011002723396548e-1_f64) * t81379 - F::new(6.0) * t12444 * t6993 + F::cast_from(0.49348022005446793095e-1_f64) * t81386 + t539 * t81011 * t568 - t6958 * t12438 + F::new(6.0) * t22656 * t3889 - F::cast_from(0.11514538467937585055e0_f64) * t81393 + F::cast_from(0.11514538467937585055e0_f64) * t81395 - t81399 - F::new(3.0) * t3882 * t22905 + F::new(12.0) * t3758 * t22653;
    t81404
}
