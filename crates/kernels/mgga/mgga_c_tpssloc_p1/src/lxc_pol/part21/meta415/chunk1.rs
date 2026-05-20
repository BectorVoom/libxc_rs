//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1931/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1931<F: Float>(t14850: F, t3316: F, t11282: F, t1694: F, t11285: F, t3377: F, t1164: F, t300: F, t4832: F) -> (F, F, F, F, F) {
    let t14852 = F::cast_from(0.16081979498692535067e2_f64) * t14850 * t3316;
    let t14853 = t11282 * t1694;
    let t14854 = t11285 * t3377;
    let t14855 = t14853 * t14854;
    let t14857 = F::cast_from(0.10254018858216406658e4_f64) * t1164 * t14855;
    let t14858 = t300 * t4832;
    (t14852, t14854, t14855, t14857, t14858)
}
