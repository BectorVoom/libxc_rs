//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1107/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1107<F: Float>(t1163: F, t5815: F, t508: F, t5935: F, t5709: F, t18295: F, t1845: F, t18551: F, t5909: F, t1811: F, t198: F, t206: F) -> (F, F, F, F, F, F) {
    let t18707 = t1163 * t5815;
    let t18710 = t508 * t5935;
    let t18711 = t18710 * t5709;
    let t18714 = t1845 * t18295;
    let t18717 = t5909 * t18551;
    let t18728 = t198 * t206 * t1811;
    (t18707, t18710, t18711, t18714, t18717, t18728)
}
