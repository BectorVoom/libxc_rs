//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1333/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1333<F: Float>(t12898: F, t19476: F, t12970: F, t18454: F, t12974: F, t13009: F, t12883: F, t18444: F, t339: F, t4419: F, t790: F, t1246: F, t136: F, t1693: F) -> (F, F, F, F, F, F, F) {
    let t65580 = t19476 * t12898;
    let t65582 = t18454 * t12970;
    let t65584 = t18454 * t12974;
    let t65586 = t19476 * t13009;
    let t65588 = t18454 * t12883;
    let t65592 = t339 * t18444 * t790 * t4419;
    let t65595 = t1693 * t1246 * t136;
    (t65580, t65582, t65584, t65586, t65588, t65592, t65595)
}
