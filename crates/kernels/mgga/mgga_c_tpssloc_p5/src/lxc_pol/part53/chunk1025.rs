//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1025/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1025<F: Float>(t116476: F, t116492: F, t123714: F, t123719: F, t1484: F, t1530: F, t16596: F, t1877: F, t193: F, t202: F, t2522: F, t25365: F, t25374: F, t32030: F, t32034: F, t33991: F, t4119: F, t4255: F, t4303: F, t4314: F, t776: F, t868: F, t870: F, t8744: F) -> F {
    let t123798 = t123714 * t193 * t202 * t870 - t116476 * t1530 * t1877 + F::new(2.0) * t116492 * t1877 * t25374 - t123719 * t1877 * t868 + F::new(3.0) * t1484 * t2522 * t32030 - F::new(3.0) * t16596 * t2522 * t32034 - t1877 * t32034 * t4303 - F::new(3.0) * t2522 * t25365 * t32034 + F::new(3.0) * t2522 * t33991 * t776 + F::new(3.0) * t2522 * t4119 * t8744 + F::new(6.0) * t4255 * t4314 * t8744;
    t123798
}
