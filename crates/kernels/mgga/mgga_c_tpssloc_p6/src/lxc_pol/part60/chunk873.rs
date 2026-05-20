//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 873/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk873<F: Float>(t5371: F, t8326: F, t1458: F, t3941: F, t7042: F, t7468: F, t1874: F, t27188: F, t1441: F, t1873: F) -> (F, F, F, F, F, F, F, F) {
    let t33191 = t5371 * t8326;
    let t33192 = F::new(0.135e2) * t33191;
    let t33193 = t8326 * t1458;
    let t33194 = t3941 * t33193;
    let t33195 = F::new(27.0) * t33194;
    let t33199 = F::new(2.0) * t7042 * t7468;
    let t33208 = F::new(2.0) * t27188 * t1874;
    let t33211 = t1441 * t1873;
    (t33191, t33192, t33193, t33194, t33195, t33199, t33208, t33211)
}
