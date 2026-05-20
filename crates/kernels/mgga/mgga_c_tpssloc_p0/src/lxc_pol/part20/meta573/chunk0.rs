//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2136/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2136<F: Float>(t2402: F, t976: F, t973: F, t979: F, t2955: F, t2967: F, t986: F, t3010: F, t698: F, t10327: F, t135: F, t10286: F, t2960: F) -> (F, F, F, F, F, F, F) {
    let t42891 = t2402 * t976;
    let t42893 = t973 * t42891 * t979;
    let t42895 = t2955 * t2967;
    let t42903 = t973 * t2402 * t986;
    let t42906 = t973 * t698 * t3010;
    let t42909 = t973 * t135 * t10327;
    let t42911 = t2960 * t10286;
    (t42891, t42893, t42895, t42903, t42906, t42909, t42911)
}
