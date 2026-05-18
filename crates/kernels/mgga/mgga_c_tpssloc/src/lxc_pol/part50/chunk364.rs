//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 364/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk364<F: Float>(t1589: F, t959: F, t1409: F, t978: F, t977: F, t1554: F, t906: F) -> (F, F, F, F) {
    let t1591 = F::new(0.5848223622634646207e0) * t959 * t1589;
    let t1592 = t978 * t1409;
    let t1593 = t977 * t1592;
    let t1597 = t906 / F::new(6.0) + t1554 / F::new(6.0);
    (t1591, t1592, t1593, t1597)
}
