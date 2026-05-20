//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2524/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2524<F: Float>(t10190: F, t13861: F, t2986: F, t13559: F, t13779: F, t10189: F, t4540: F, t2990: F, t42771: F, t4514: F, t43057: F, t13913: F, t2960: F) -> (F, F, F, F, F, F, F) {
    let t48030 = t2986 * t10190 * t13861;
    let t48044 = t2986 * t13779 * t13559;
    let t48046 = t10189 * t4540;
    let t48048 = t2986 * t48046 * t2990;
    let t48052 = t2986 * t42771 * t4514;
    let t48061 = t2986 * t43057 * t4514;
    let t48063 = t2960 * t13913;
    (t48030, t48044, t48046, t48048, t48052, t48061, t48063)
}
