//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1315/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1315<F: Float>(t43776: F, t43837: F, t43839: F, t43842: F, t43845: F, t43848: F, t43851: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t1174: F, t1186: F, t2402: F, t11498: F, t135: F) -> (F, F, F) {
    let t44466 = 220.0 / 81.0 * t43776;
    let t44470 = 8.0 / 3.0 * t43837 + 4.0 / 9.0 * t43839 - 8.0 / 9.0 * t43842 + 2.0 * t43845 - 4.0 * t43848 - t43851 / 6.0 + 10.0 / 27.0 * t43855 + 16.0 / 81.0 * t43857 - t44466 + 160.0 / 81.0 * t43859 - 10.0 / 9.0 * t43861 - 20.0 / 9.0 * t43863;
    let t44478 = t1174 * t2402 * t1186;
    let t44481 = t1174 * t135 * t11498;
    (t44470, t44478, t44481)
}
