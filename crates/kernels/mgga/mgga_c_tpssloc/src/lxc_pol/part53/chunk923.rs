//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 923/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk923<F: Float>(t25: F, t7844: F, t1484: F, t1530: F, t1877: F, t193: F, t202: F, t2522: F, t32034: F, t32047: F, t33990: F, t7114: F, t870: F, t8744: F, t8748: F) -> (F, F) {
    let t34004 = t25 * t7844;
    let t34030 = t193 * t202 * t33990 * t870 + F::new(3.0) * t1484 * t2522 * t8744 - F::new(3.0) * t1484 * t2522 * t8748 - t1530 * t1877 * t32034 + F::new(2.0) * t1530 * t1877 * t32047 - F::new(2.0) * t1877 * t7114 * t7844;
    (t34004, t34030)
}
