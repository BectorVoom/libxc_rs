//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 885/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk885<F: Float>(t1877: F, t193: F, t202: F, t2522: F, t32029: F, t32034: F, t32047: F, t7109: F, t7114: F, t776: F, t868: F, t870: F, t8744: F, t8748: F) -> F {
    let t32071 = t193 * t202 * t32029 * t870 - t1877 * t32034 * t868 + F::new(2.0) * t1877 * t32047 * t868 - F::new(2.0) * t1877 * t7109 * t7114 + F::new(3.0) * t2522 * t776 * t8744 - F::new(3.0) * t2522 * t776 * t8748;
    t32071
}
