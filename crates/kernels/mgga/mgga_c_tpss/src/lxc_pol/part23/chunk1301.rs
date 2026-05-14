//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1301/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1301<F: Float>(t65570: F, t12996: F, t5716: F, t12978: F, t18454: F, t12982: F, t12986: F, t12898: F, t19476: F, t12970: F, t12974: F, t13009: F, t12883: F, t18444: F, t339: F, t4419: F, t790: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t65571 = 7.0 / 72.0 * t65570;
    let t65572 = t5716 * t12996;
    let t65574 = t18454 * t12978;
    let t65576 = t18454 * t12982;
    let t65578 = t18454 * t12986;
    let t65580 = t19476 * t12898;
    let t65582 = t18454 * t12970;
    let t65584 = t18454 * t12974;
    let t65586 = t19476 * t13009;
    let t65588 = t18454 * t12883;
    let t65592 = t339 * t18444 * t790 * t4419;
    (t65571, t65572, t65574, t65576, t65578, t65580, t65582, t65584, t65586, t65588, t65592)
}
