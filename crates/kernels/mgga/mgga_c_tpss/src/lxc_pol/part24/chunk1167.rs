//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1167/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1167<F: Float>(t1265: F, t6262: F, t18490: F, t1232: F, t1656: F, t520: F, t18497: F, t1266: F, t1657: F, t1775: F, t18474: F, t18483: F, t18496: F, t19498: F, t19500: F, t19507: F, t19509: F, t4494: F, t4517: F, t538: F, t5734: F, t5739: F, t5742: F, t5748: F, t5751: F, t6260: F, t6263: F) -> (F, F, F, F) {
    let t19516 = t6262 * t1265;
    let t19517 = t18490 * t19516;
    let t19521 = t1656 * t1232 * t520;
    let t19522 = t18497 * t19521;
    let t19525 = -t1266 * t19500 - t1657 * t18474 - t1775 * t19507 + 2.0 * t18483 * t6263 - 2.0 * t18496 * t19522 + t19498 * t538 + 2.0 * t19509 * t5742 + t19509 * t5748 - 6.0 * t19517 * t5739 + 2.0 * t4494 * t5734 - t4517 * t5734 - t5751 * t6260;
    (t19517, t19521, t19522, t19525)
}
