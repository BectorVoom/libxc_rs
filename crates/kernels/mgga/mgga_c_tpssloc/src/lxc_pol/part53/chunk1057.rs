//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1057/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1057<F: Float>(t117357: F, t117359: F, t124603: F, t124609: F, t124612: F, t124635: F, t124668: F, t1396: F, t1398: F, t1852: F, t2099: F, t27286: F, t32311: F, t34102: F, t5364: F, t7223: F, t7240: F, t7946: F, t7961: F, t8822: F) -> F {
    let t124671 = F::new(2.0) * t7946 * t7240 + F::new(2.0) * t124603 + F::new(2.0) * t7223 * t7961 + F::new(2.0) * t2099 * t27286 + t117357 + t117359 + t124609 + t5364 * t8822 + t1852 * t32311 + t124612 + t1396 * t34102 + t1398 * (t124635 + t124668);
    t124671
}
