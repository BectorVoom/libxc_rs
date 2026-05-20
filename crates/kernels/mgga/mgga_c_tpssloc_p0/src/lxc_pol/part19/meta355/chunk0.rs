//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1284/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1284<F: Float>(t41654: F, t242: F, t281: F, t283: F, t136: F, t2826: F, t41705: F, t10304: F, t41693: F, t41715: F, t908: F, t41644: F) -> (F, F, F, F, F, F, F) {
    let t41959 = F::cast_from(0.31310740740740740741e1_f64) * t41654;
    let t41961 = t281 * t242 * t283;
    let t41962 = F::cast_from(0.13490888888888888889e1_f64) * t41961;
    let t41964 = t136 * t2826 * t41705;
    let t41967 = t136 * t10304 * t41693;
    let t41970 = t136 * t908 * t41715;
    let t41973 = t136 * t908 * t41644;
    (t41959, t41961, t41962, t41964, t41967, t41970, t41973)
}
