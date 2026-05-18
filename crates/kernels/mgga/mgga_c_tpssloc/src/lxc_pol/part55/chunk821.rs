//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 821/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk821<F: Float>(t1375: F, t2016: F, t568: F, t6958: F, t8457: F, t8461: F, t8471: F, t8476: F, t8486: F) -> F {
    let t8488 = F::new(2.0) * t1375 * t8476 - t1375 * t8486 - F::new(2.0) * t2016 * t6958 + t568 * t8471 + t8457 - t8461;
    t8488
}
