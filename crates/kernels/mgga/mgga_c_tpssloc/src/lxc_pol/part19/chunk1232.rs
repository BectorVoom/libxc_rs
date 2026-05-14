//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1232/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1232<F: Float>(t10225: F, t2960: F, t10213: F, t135: F, t10218: F, t973: F, t344: F, t41687: F, t10236: F, t10913: F, t41831: F, t41833: F, t41836: F, t41839: F, t41842: F, t41887: F, t41889: F, t41892: F, t41964: F, t41967: F, t41970: F) -> (F, F, F, F, F) {
    let t42968 = t2960 * t10225;
    let t42972 = t135 * t10213;
    let t42974 = t973 * t42972 * t10218;
    let t42976 = t344 * t41687;
    let t42985 = t10236 * t10913;
    let t43000 = -20.0 / 9.0 * t41831 - 8.0 / 3.0 * t41833 + 8.0 / 3.0 * t41887 - 4.0 / 9.0 * t41889 + 2.0 * t41836 - 2.0 * t41892 + t41839 / 6.0 + 2.0 / 9.0 * t41964 + 4.0 / 9.0 * t41967 - 4.0 * t41842 + 6.0 * t41970;
    (t42968, t42974, t42976, t42985, t43000)
}
