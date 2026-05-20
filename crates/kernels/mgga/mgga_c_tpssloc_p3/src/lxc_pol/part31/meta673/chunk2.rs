//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2026/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2026<F: Float>(t84555: F, t84558: F, t91398: F, t91400: F, t91406: F, t93762: F, t93763: F, t97435: F, t97437: F, t97439: F, t97444: F, t97447: F, t97450: F, t97453: F, t97456: F, t97459: F, t97461: F, t97463: F) -> F {
    let t102746 = -F::cast_from(0.96894614625936938048e-2_f64) * t97435 - t97437 / F::new(24.0) + F::cast_from(0.16956557559538964158e-1_f64) * t97439 - F::new(35.0) / F::new(54.0) * t91398 - F::cast_from(0.27130492095262342653e0_f64) * t91400 + t93762 + t93763 - t91406 + F::cast_from(0.28260929265898273597e-2_f64) * t97444 + F::cast_from(0.33913115119077928316e-1_f64) * t97447 + F::cast_from(0.16956557559538964158e-1_f64) * t97450 - t84555 + t84558 - t97453 / F::new(2.0) + t97456 / F::new(4.0) - F::cast_from(0.13565246047631171326e0_f64) * t97459 - t97461 / F::new(128.0) + F::cast_from(0.28260929265898273597e-2_f64) * t97463;
    t102746
}
