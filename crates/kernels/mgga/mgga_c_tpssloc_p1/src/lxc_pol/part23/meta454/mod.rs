//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1311;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1312;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta454<F: Float>(t16693: F, t20749: F, t46376: F, t16689: F, t5597: F, t39585: F, t39590: F, t39593: F, t41254: F, t75943: F, t75950: F, t75951: F, t75952: F, t185: F, t707: F, t75912: F, t58984: F, t46433: F, t46439: F, t1409: F, t4194: F, t67469: F, t59013: F, t12939: F, t16716: F, t5398: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t76017, t76018, t76020, t76021) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1311::<F>(t16693, t20749, t46376, t16689, t5597, t39585, t39590, t39593, t41254, t75943, t75950, t75951, t75952);
        let (t76024, t76025, t76026, t76027, t76030, t76031, t76034) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1312::<F>(t185, t707, t75912, t58984, t46433, t46439, t1409, t4194, t67469, t59013, t12939, t16716, t5398);
    (t76017, t76018, t76020, t76021, t76024, t76025, t76026, t76027, t76030, t76031, t76034)
}
