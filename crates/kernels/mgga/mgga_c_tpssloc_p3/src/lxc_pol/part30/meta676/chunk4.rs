//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2111/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2111<F: Float>(t12571: F, t26083: F, t1865: F, t22523: F, t22554: F, t26051: F, t26055: F, t26067: F, t26090: F, t27976: F, t6490: F, t6492: F, t7442: F, t7446: F, t96517: F, t96521: F, t96529: F, t96532: F, t96535: F) -> F {
    let t96538 = t12571 * t26083;
    let t96545 = F::new(5.0) / F::new(6.0) * t22554 * t27976 + F::new(5.0) / F::new(6.0) * t22523 * t27976 + F::new(5.0) / F::new(6.0) * t6490 * t96517 + F::new(5.0) / F::new(6.0) * t6490 * t96521 + F::new(5.0) / F::new(3.0) * t26051 * t26067 + F::new(2.0) / F::new(3.0) * t26055 * t7446 + F::new(5.0) / F::new(6.0) * t96529 * t6492 + F::new(5.0) / F::new(6.0) * t96532 * t6492 + t96535 * t1865 / F::new(3.0) + F::new(5.0) / F::new(3.0) * t96538 * t6492 + F::new(2.0) / F::new(3.0) * t26055 * t7442 + F::new(5.0) / F::new(3.0) * t26051 * t26090;
    t96545
}
