//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1160/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1160<F: Float>(t38: F, t6085: F, t1981: F, t1680: F, t18335: F, t18342: F, t19388: F, t19393: F, t19396: F, t19404: F, t19408: F, t19411: F, t19414: F, t19417: F, t5487: F, t5489: F, t5492: F, t5503: F, t5507: F, t6077: F, t6080: F, t6091: F) -> (F, F, F) {
    let t19424 = t38 * t6085;
    let t19425 = t1981 * t19424;
    let t19428 = 5.0 / 6.0 * t5487 * t19388 + t5492 * t6091 / 3.0 + 5.0 / 6.0 * t19393 * t5489 + t19396 * t1680 / 3.0 + 5.0 / 6.0 * t18335 * t6077 + 5.0 / 6.0 * t18342 * t6077 + 5.0 / 6.0 * t5487 * t19404 + 5.0 / 6.0 * t5487 * t19408 + t19411 * t1680 / 3.0 + t19414 * t1680 / 3.0 + t19417 * t1680 / 3.0 + t6080 * t5503 / 3.0 + t6080 * t5507 / 3.0 + 5.0 / 6.0 * t19425 * t5489;
    (t19424, t19425, t19428)
}
