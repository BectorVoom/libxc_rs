//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1132/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1132<F: Float>(t33: F, t259: F, t479: F, t14432: F, t15476: F, t16022: F, t1006: F, t1157: F, t1289: F, t13335: F, t13603: F, t14440: F, t1497: F, t1594: F, t3431: F, t3735: F, t4333: F, t4579: F, t481: F, t4818: F, t5059: F, t5306: F, t57: F, t581: F, t826: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t16024 = piecewise3(t480, t15476 + t16022, t14432);
    let t16036 = piecewise3(t386, t14432 * t33 / 2.0 + t4818 * t1006 / 2.0 + t3735 * t1497 - t14440 + t826 * t5059 / 2.0 + t259 * t13603 / 2.0, t16024 * t57 / 2.0 - t5306 * t581 / 2.0 - t4333 * t1289 - t1594 * t3431 - t1157 * t4579 / 2.0 - t481 * t13335 / 2.0);
    (t16036,)
}
