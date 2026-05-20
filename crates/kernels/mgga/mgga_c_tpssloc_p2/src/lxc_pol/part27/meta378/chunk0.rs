//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1550/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1550<F: Float>(t14142: F, t4582: F, t12648: F, t4583: F, t13559: F, t977: F, t2960: F, t4603: F, t1606: F, t698: F, t973: F, t1043: F, t2770: F) -> (F, F, F, F, F, F, F) {
    let t14143 = t4582 * t14142;
    let t14146 = t4583 * t12648;
    let t14147 = t4582 * t14146;
    let t14152 = t977 * t13559;
    let t14158 = t2960 * t4603 / F::new(162.0);
    let t14159 = t698 * t1606;
    let t14160 = t973 * t14159;
    let t14164 = t1043 * t2770;
    (t14143, t14147, t14152, t14158, t14159, t14160, t14164)
}
