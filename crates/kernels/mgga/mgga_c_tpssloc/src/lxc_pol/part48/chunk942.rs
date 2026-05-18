//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 942/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk942<F: Float>(t20173: F, t31817: F, t1874: F, t91854: F, t23938: F, t6525: F, t1873: F, t2311: F, t2040: F, t2314: F, t31744: F, t4034: F) -> (F, F, F, F, F, F, F) {
    let t114531 = F::new(54.0) * t20173 * t31817;
    let t114541 = F::new(4.0) * t91854 * t1874;
    let t114543 = F::new(4.0) * t23938 * t6525;
    let t114552 = t2311 * t1873;
    let t114554 = F::new(2.0) * t114552 * t2040;
    let t114559 = F::new(4.0) * t2314 * t31744;
    let t114561 = F::new(4.0) * t4034 * t31744;
    (t114531, t114541, t114543, t114552, t114554, t114559, t114561)
}
