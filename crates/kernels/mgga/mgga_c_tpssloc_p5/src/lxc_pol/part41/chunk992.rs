//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 992/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk992<F: Float>(t13969: F, t4584: F, t1041: F, t4589: F, t2960: F, t4603: F, t1606: F, t698: F, t973: F, t1043: F, t2770: F, t10277: F, t3061: F) -> (F, F, F, F, F, F) {
    let t14134 = t13969 * t4584;
    let t14136 = t1041 * t14134 / F::cast_from(1728.0_f64);
    let t14137 = t13969 * t4589;
    let t14139 = F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t1041 * t14137;
    let t14158 = t2960 * t4603 / F::cast_from(162.0_f64);
    let t14159 = t698 * t1606;
    let t14160 = t973 * t14159;
    let t14164 = t1043 * t2770;
    let t14172 = t3061 * t10277;
    (t14136, t14139, t14158, t14160, t14164, t14172)
}
