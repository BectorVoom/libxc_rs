//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1340/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1340<F: Float>(t42667: F, t5965: F, t1861: F, t19232: F, t19404: F, t19408: F, t19414: F, t19417: F, t20719: F, t20777: F, t20780: F, t21116: F, t5489: F, t6077: F, t6080: F, t63492: F, t63498: F, t6475: F, t68122: F, t69097: F, t69165: F) -> (F,) {
    let t72914 = t42667 * t5965;
    let t72919 = 2.0 / 3.0 * t19414 * t6475 + 2.0 / 3.0 * t19417 * t6475 + 2.0 / 3.0 * t6080 * t20777 + 2.0 / 3.0 * t6080 * t20780 + 5.0 / 3.0 * t68122 * t6077 + 5.0 / 3.0 * t20719 * t19404 + 5.0 / 3.0 * t20719 * t19408 - 5.0 * t63492 * t21116 - 5.0 * t63498 * t21116 - 5.0 * t19232 * t69097 + 5.0 / 6.0 * t72914 * t5489 + t69165 * t1861 / 3.0;
    (t72919,)
}
