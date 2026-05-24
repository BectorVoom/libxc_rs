//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 719/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk719<F: Float>(t1600: F, t645: F, t1342: F, t2112: F, t2335: F, t1398: F, t823: F, t198: F, t205: F) -> (F, F, F, F, F) {
    let t3542 = t1600 * t645;
    let t3546 = F::new(4.0) * t2112 * t1342;
    let t3547 = F::new(4.0) * t2335;
    let t3548 = t1398 * t823;
    let t3552 = t198 * t205;
    (t3542, t3546, t3547, t3548, t3552)
}
