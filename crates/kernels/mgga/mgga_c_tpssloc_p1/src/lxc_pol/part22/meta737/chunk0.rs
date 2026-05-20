//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2420/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2420<F: Float>(t49486: F, t5695: F, t10655: F, t21253: F, t17521: F, t48763: F, t21347: F, t300: F, t961: F, t10702: F, t14395: F, t5726: F, t912: F) -> (F, F, F, F, F) {
    let t69003 = F::new(6.0) * t49486 * t5695;
    let t69005 = F::new(6.0) * t10655 * t21253;
    let t69011 = F::cast_from(0.2894756309764656312e3_f64) * t48763 * t17521;
    let t69012 = t300 * t21347;
    let t69014 = F::cast_from(0.5848223622634646207e0_f64) * t69012 * t961;
    let t69018 = F::cast_from(0.1551780387578202009e4_f64) * t10702 * t5726 * t14395 * t912;
    (t69003, t69005, t69011, t69014, t69018)
}
