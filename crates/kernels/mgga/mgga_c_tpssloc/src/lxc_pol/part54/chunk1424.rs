//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1424/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1424<F: Float>(t115545: F, t26331: F, t26333: F, t114225: F, t115572: F, t120561: F, t120566: F, t120569: F, t1375: F, t1807: F, t2016: F, t22670: F, t26472: F, t31584: F, t31601: F, t3887: F, t5210: F, t5215: F, t568: F, t7194: F, t7213: F, t7749: F, t7925: F, t8617: F, t93313: F) -> F {
    let t122304 = t26331 * t115545 * t26333;
    let t122319 = F::new(0.41123351671205660912e-2) * t115572 + t114225 - t7194 * t26472 + F::new(0.49348022005446793095e-1) * t122304 - t120561 - t120566 - t93313 * t2016 + t5210 * t8617 * t568 + t1807 * t31584 * t568 + F::new(2.0) * t22670 * t7925 + F::new(2.0) * t1375 * t3887 * t7213 * t7749 + t120569 + F::new(2.0) * t5215 * t31601;
    t122319
}
