//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1976/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1976<F: Float>(t3572: F, t5002: F, t3523: F, t5005: F, t5019: F, t5024: F, t11147: F, t11778: F, t14165: F, t4582: F, t1735: F, t3252: F) -> (F, F, F, F, F, F, F, F) {
    let t15446 = t5002 * t3572 / F::new(2304.0);
    let t15448 = t5005 * t3523 / F::new(3456.0);
    let t15450 = t5019 * t3572 / F::new(432.0);
    let t15452 = t5024 * t3523 / F::new(648.0);
    let t15453 = t11778 * t11147;
    let t15454 = t15453 * t14165;
    let t15455 = t4582 * t15454;
    let t15458 = t1735 * t3252;
    (t15446, t15448, t15450, t15452, t15453, t15454, t15455, t15458)
}
