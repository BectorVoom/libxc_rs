//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 778/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk778<F: Float>(t33: F, t1497: F, t3289: F, t2: F, t493: F, t1006: F, t555: F, t162: F, t4367: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t4368 = t3289 * t1497;
    let t4371 = t493 * t2;
    let t4375 = piecewise3(t34, 0.0, 4.0 / 9.0 * t4368 * t1006 - 8.0 / 3.0 * t4371 * t555);
    let t4377 = (t4367 + t4375) * t162;
    (t4368, t4371, t4377)
}
