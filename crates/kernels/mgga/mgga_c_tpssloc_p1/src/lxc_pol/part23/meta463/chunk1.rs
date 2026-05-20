//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1355/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1355<F: Float>(t13769: F, t17794: F, t17804: F, t2986: F, t340: F, t343: F, t4510: F, t4531: F, t61310: F, t61313: F, t69548: F, t69647: F, t69683: F, t69686: F, t69691: F, t69699: F, t69727: F, t69739: F, t69746: F, t76593: F, t76901: F, t76922: F, t973: F, t974: F) -> F {
    let t76943 = -F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t17804 * t17794 - F::cast_from(0.13333333333333333333e-1_f64) * t2986 * t4510 * t76593 + F::cast_from(0.88888888888888888886e-2_f64) * t2986 * t13769 * t69548 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t974 * t340 * (t76901 + t76922) * t343 - F::cast_from(0.22222222222222222221e-2_f64) * t69683 - F::cast_from(0.11111111111111111111e-2_f64) * t69686 - F::cast_from(0.11111111111111111111e-2_f64) * t69691 - F::cast_from(0.14814814814814814815e-2_f64) * t69699 - F::cast_from(0.29629629629629629628e-2_f64) * t69727 + F::cast_from(0.37037037037037037036e-3_f64) * t69739 + F::cast_from(0.66666666666666666664e-2_f64) * t2986 * t4531 * t69746 - F::cast_from(0.44444444444444444444e-2_f64) * t2986 * t13769 * t69647 + F::cast_from(0.11111111111111111111e-2_f64) * t61310 + F::cast_from(0.11111111111111111111e-2_f64) * t61313;
    t76943
}
