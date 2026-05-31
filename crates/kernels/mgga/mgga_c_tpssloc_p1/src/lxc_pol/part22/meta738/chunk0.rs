//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2422/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2422<F: Float>(t17517: F, t49226: F, t21347: F, t942: F, t10765: F, t14266: F, t1569: F, t17428: F, t21259: F, t4434: F, t49427: F, t5743: F, t5759: F, t59962: F, t68762: F, t68764: F, t68767: F, t68769: F, t68771: F, t68773: F, t68775: F, t68883: F, t68885: F, t952: F) -> (F, F) {
    let t69036 = F::cast_from(18.0_f64) * t49226 * t17517;
    let t69047 = t21347 * t942;
    let t69050 = F::cast_from(3.0_f64) * t59962 * t1569 + F::cast_from(3.0_f64) * t17428 * t4434 + F::cast_from(3.0_f64) * t14266 * t5759 - t68762 - t68764 - t68767 - t68769 - t68771 + t68773 - t68775 - t68883 - t68885 - F::cast_from(6.0_f64) * t49427 * t5743 + F::cast_from(6.0_f64) * t10765 * t21259 + F::cast_from(0.5848223622634646207e0_f64) * t69047 * t952;
    (t69036, t69050)
}
