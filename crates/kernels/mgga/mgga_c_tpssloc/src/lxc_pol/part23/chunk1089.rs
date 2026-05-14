//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1089/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1089<F: Float>(t1489: F, t41083: F, t133: F, t1484: F, t41214: F, t6600: F, t1512: F, t41362: F, t4166: F, t9666: F, t9973: F, t10024: F, t1500: F, t9670: F, t9600: F, t1540: F, t9698: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46790 = t41083 * t1489;
    let t46806 = t41214 * t133 * t6600 * t1484;
    let t46876 = t41362 * t1512;
    let t46881 = t4166 * t9666;
    let t46957 = t4166 * t9973;
    let t47047 = t1500 * t10024;
    let t47092 = t4166 * t9670;
    let t47275 = t4166 * t9600;
    let t47787 = t9698 * t1540;
    (t46790, t46806, t46876, t46881, t46957, t47047, t47092, t47275, t47787)
}
