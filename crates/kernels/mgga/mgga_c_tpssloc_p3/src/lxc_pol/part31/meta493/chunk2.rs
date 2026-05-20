//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1683/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1683<F: Float>(t27170: F, t510: F, t1458: F, t7156: F, t1983: F, t2040: F, t2314: F, t26179: F, t27145: F, t27147: F, t27150: F, t27163: F, t4028: F, t4034: F, t652: F, t7050: F, t7057: F, t7061: F, t7458: F, t7796: F, t7806: F) -> (F, F, F) {
    let t27171 = t510 * t27170;
    let t27180 = t7156 * t1458;
    let t27183 = t1983 * t27145 - F::new(2.0) * t2040 * t26179 - F::new(2.0) * t2314 * t7796 - F::new(2.0) * t2314 * t7806 - F::new(2.0) * t27147 * t652 - F::new(2.0) * t27150 * t652 - F::new(2.0) * t27163 * t652 - F::new(2.0) * t27171 * t652 - F::new(2.0) * t27180 * t652 - F::new(2.0) * t4028 * t7061 - F::new(2.0) * t4034 * t7796 - F::new(2.0) * t4034 * t7806 - F::new(2.0) * t7050 * t7458 - F::new(2.0) * t7057 * t7458;
    (t27171, t27180, t27183)
}
