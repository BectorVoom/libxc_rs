//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 989/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk989<F: Float>(t13969: F, t4599: F, t3039: F, t3069: F, t4669: F, t10231: F, t4338: F, t973: F, t4595: F, t3130: F, t3048: F, t4571: F) -> (F, F, F, F, F) {
    let t13970 = t13969 * t4599;
    let t13972 = t3039 * t13970 / F::cast_from(2304.0_f64);
    let t13995 = t4669 * t3069;
    let t13998 = t10231 * t4338;
    let t14000 = t973 * t13998 / F::cast_from(324.0_f64);
    let t14025 = t13969 * t4595;
    let t14027 = t3130 * t14025 / F::cast_from(1152.0_f64);
    let t14049 = t3048 * t4571 / F::cast_from(648.0_f64);
    (t13972, t13995, t14000, t14027, t14049)
}
