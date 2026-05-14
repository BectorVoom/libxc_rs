//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1285/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1285<F: Float>(t13442: F, t76: F, t1981: F, t21157: F, t38: F, t4622: F, t619: F, t77: F, t1675: F, t1678: F, t19368: F, t19380: F, t19388: F, t19396: F, t19425: F, t21159: F, t21162: F, t21165: F, t21166: F, t5483: F, t5487: F, t5489: F, t5492: F, t5502: F, t6086: F, t6090: F, t6091: F) -> (F,) {
    let t69338 = t76 * t13442;
    let t69345 = t1981 * t38 * t21157;
    let t69355 = t77 * t4622 * t619;
    let t69360 = -t1675 * t19368 * t6090 / 3.0 - t1675 * t6086 * t19380 / 3.0 - t5483 * t21166 / 6.0 - t1675 * t5502 * t21165 / 6.0 - t1675 * t1678 * t69338 / 6.0 + 2.0 / 3.0 * t19396 * t6091 + 5.0 / 6.0 * t69345 * t5489 + t5492 * t21159 / 3.0 + 5.0 / 3.0 * t19425 * t19388 + 2.0 / 3.0 * t5492 * t21162 + 5.0 / 6.0 * t5487 * t69355 + t5492 * t21166 / 3.0;
    (t69360,)
}
