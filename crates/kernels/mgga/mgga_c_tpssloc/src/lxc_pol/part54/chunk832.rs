//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 832/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk832<F: Float>(t1825: F, t7208: F, t553: F, t7918: F, t1336: F, t1814: F, t2089: F, t544: F, t7202: F, t7204: F, t7734: F, t7738: F, t7742: F) -> (F, F, F) {
    let t7932 = t7208 * t1825;
    let t7934 = t553 * t7918;
    let t7936 = -t7202 - F::new(0.3289868133696452873e-1) * t7734 - t7204 - F::new(0.16449340668482264365e-1) * t7738 + F::new(0.16449340668482264365e-1) * t7742 + t1814 * t2089 - t1336 * t7932 + t544 * t7934;
    (t7932, t7934, t7936)
}
