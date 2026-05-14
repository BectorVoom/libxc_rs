//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1211/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1211<F: Float>(t121004: F, t1874: F, t2035: F, t4072: F, t115241: F, t120986: F, t120991: F, t120993: F, t120995: F, t120998: F, t121003: F, t1459: F, t2314: F, t26906: F, t33204: F, t4034: F, t652: F, t6862: F, t7801: F, t8450: F) -> (F, F) {
    let t121006 = 2.0 * t121004 * t1874;
    let t121007 = t2035 * t4072;
    let t121009 = 2.0 * t121007 * t1874;
    let t121017 = -2.0 * t652 * t6862 * t7801 - 2.0 * t115241 * t1459 - 2.0 * t2314 * t33204 + 3.0 * t26906 * t8450 - 2.0 * t33204 * t4034 - t120986 + t120991 - t120993 - t120995 - t120998 - t121003 - t121006 - t121009;
    (t121007, t121017)
}
