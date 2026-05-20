//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1969/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1969<F: Float>(t84896: F, t84897: F, t87304: F, t87306: F, t92626: F, t92627: F, t92630: F, t98715: F, t98717: F, t98719: F, t98721: F, t98723: F, t98725: F, t98728: F, t98731: F, t98733: F, t98736: F, t98738: F) -> F {
    let t101439 = -t84896 - t84897 + t92626 + t92627 - t92630 + F::new(5.0) / F::new(96.0) * t98715 - F::new(5.0) / F::new(32.0) * t98717 + F::new(5.0) / F::new(96.0) * t98719 + F::new(5.0) / F::new(192.0) * t98721 - t98723 / F::new(768.0) + F::cast_from(0.28260929265898273597e-2_f64) * t98725 - F::cast_from(0.96894614625936938048e-2_f64) * t98728 + F::cast_from(0.48447307312968469024e-2_f64) * t98731 + F::new(7.0) / F::new(288.0) * t98733 - F::new(35.0) / F::new(54.0) * t87304 - F::cast_from(0.27130492095262342653e0_f64) * t87306 + F::new(7.0) / F::new(1152.0) * t98736 + F::new(7.0) / F::new(576.0) * t98738;
    t101439
}
