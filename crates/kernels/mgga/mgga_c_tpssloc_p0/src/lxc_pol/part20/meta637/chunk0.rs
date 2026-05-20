//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2342/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2342<F: Float>(t1597: F, t43052: F, t2986: F, t2990: F, t10255: F, t13847: F, t10190: F, t13861: F, t13559: F, t13779: F, t10189: F, t4540: F) -> (F, F, F, F, F) {
    let t48019 = t43052 * t1597;
    let t48021 = t2986 * t48019 * t2990;
    let t48022 = F::cast_from(0.18518518518518518518e-3_f64) * t48021;
    let t48024 = t2986 * t13847 * t10255;
    let t48030 = t2986 * t10190 * t13861;
    let t48044 = t2986 * t13779 * t13559;
    let t48046 = t10189 * t4540;
    (t48022, t48024, t48030, t48044, t48046)
}
