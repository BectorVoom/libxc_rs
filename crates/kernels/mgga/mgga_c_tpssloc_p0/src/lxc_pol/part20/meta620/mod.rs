//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2233;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2234;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2235;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta620<F: Float>(t40748: F, t40760: F, t40764: F, t40766: F, t46282: F, t46284: F, t46286: F, t46287: F, t46288: F, t46292: F, t46293: F, t2379: F, t868: F, t4199: F, t9722: F, t12887: F, t172: F, t763: F, t12858: F, t2535: F, t40794: F, t40804: F, t40806: F, t12606: F, t707: F, t751: F, t40808: F, t2749: F, t776: F, t12915: F, t2522: F, t39549: F, t40797: F, t40799: F, t40801: F, t40803: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46294, t46298) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2233::<F>(t40748, t40760, t40764, t40766, t46282, t46284, t46286, t46287, t46288, t46292, t46293, t2379, t868);
        let (t46303, t46309, t46311, t46313, t46314, t46315, t46317) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2234::<F>(t4199, t9722, t12887, t172, t763, t12858, t2535, t40794, t40804, t40806, t12606, t707, t751);
        let (t46318, t46319, t46324) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2235::<F>(t46317, t40808, t2749, t776, t12915, t2522, t39549, t40797, t40799, t40801, t40803, t46313, t46314, t46315);
    (t46294, t46298, t46303, t46309, t46311, t46313, t46314, t46315, t46318, t46319, t46324)
}
