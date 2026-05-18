//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1322/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1322<F: Float>(t16673: F, t6613: F, t28359: F, t838: F, t23069: F, t5572: F, t23062: F, t28383: F, t5568: F, t81956: F, t28389: F, t81963: F) -> (F, F, F, F, F, F) {
    let t98684 = t16673 * t6613;
    let t98690 = t28359 * t838;
    let t98694 = t23069 * t5572;
    let t98696 = t23062 * t28383;
    let t98709 = t81956 * t5568;
    let t98711 = t81963 * t28389;
    (t98684, t98690, t98694, t98696, t98709, t98711)
}
