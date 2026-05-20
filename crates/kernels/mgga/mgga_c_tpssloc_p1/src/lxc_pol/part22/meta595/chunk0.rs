//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2115/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2115<F: Float>(t10469: F, t1603: F, t11058: F, t11045: F, t11064: F, t1597: F, t43052: F, t2986: F, t2990: F, t10189: F, t4540: F, t4542: F, t698: F, t973: F) -> (F, F, F, F, F, F, F, F) {
    let t47840 = t1603 * t10469;
    let t47841 = t47840 * t11058;
    let t47853 = t47840 * t11045;
    let t47857 = t47840 * t11064;
    let t48019 = t43052 * t1597;
    let t48021 = t2986 * t48019 * t2990;
    let t48022 = F::cast_from(0.18518518518518518518e-3_f64) * t48021;
    let t48046 = t10189 * t4540;
    let t48066 = t973 * t698 * t4542;
    (t47840, t47841, t47853, t47857, t48019, t48022, t48046, t48066)
}
