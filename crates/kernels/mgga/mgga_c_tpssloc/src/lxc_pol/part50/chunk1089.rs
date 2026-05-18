//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1089/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1089<F: Float>(t1985: F, t32769: F, t1375: F, t2016: F, t26366: F, t31115: F, t31129: F, t32737: F, t32758: F, t32764: F, t32766: F, t5215: F, t5321: F, t6958: F, t7729: F, t8476: F, t8486: F) -> F {
    let t32771 = F::new(0.16449340668482264365e-1) * t1985 * t32769;
    let t32780 = -t1375 * t32758 + F::new(4.0) * t1375 * t32766 - F::new(2.0) * t2016 * t26366 + F::new(2.0) * t5215 * t8476 - t5215 * t8486 + F::new(2.0) * t5321 * t8476 - t5321 * t8486 + F::new(4.0) * t6958 * t7729 + t31115 + t31129 - t32737 + t32764 - t32771;
    t32780
}
