//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 979/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk979<F: Float>(t33: F, t5335: F, t9936: F, t3225: F, t5059: F, t1006: F, t1201: F, t13603: F, t1989: F, t4388: F, t13657: F, zeta_threshold: F) -> (F,) {
    let t34 = t33 <= zeta_threshold;
    let t13658 = t9936 * t5335;
    let t13663 = t3225 * t5059;
    let t13669 = piecewise3(t34, 0.0, 8.0 / 27.0 * t13658 * t1006 + 8.0 / 9.0 * t4388 * t1989 - 2.0 / 9.0 * t13663 * t1006 + 2.0 / 3.0 * t1201 * t13603);
    let t13671 = t13657 / 2.0 + t13669 / 2.0;
    (t13671,)
}
