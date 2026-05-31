//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2117/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2117<F: Float>(t2770: F, t340: F, t2403: F, t4389: F, t4386: F, t344: F, t42308: F, t60: F, t10213: F, t134: F, t4509: F, t4540: F) -> (F, F, F, F, F, F, F, F) {
    let t48143 = t340 * t2770;
    let t48155 = t2403 * t4389;
    let t48156 = F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t48155;
    let t48157 = t2403 * t4386;
    let t48158 = F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t48157;
    let t48180 = t60 * t42308 * t344;
    let t48213 = t134 * t10213 * t344;
    let t48217 = t4509 * t4540;
    (t48143, t48155, t48156, t48157, t48158, t48180, t48213, t48217)
}
