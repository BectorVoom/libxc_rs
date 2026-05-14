//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1211/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1211<F: Float>(t18511: F, t21087: F, t1639: F, t520: F, t6255: F, t5745: F, t1768: F, t5407: F, t21086: F, t1773: F, t21052: F, t522: F, t1657: F, t1772: F, t1775: F, t18496: F, t19500: F, t19509: F, t21053: F, t21061: F, t21070: F, t21075: F, t21079: F, t21083: F, t538: F, t5433: F, t5449: F, t5734: F, t5739: F, t6260: F, t6263: F, t6268: F, t6271: F) -> (F, F, F, F, F, F) {
    let t21088 = t18511 * t21087;
    let t21092 = t6255 * t1639 * t520;
    let t21093 = t5745 * t21092;
    let t21097 = t1768 * t5407 * t520;
    let t21098 = t5745 * t21097;
    let t21100 = t21086 * t520;
    let t21101 = t5745 * t21100;
    let t21104 = t1773 * t522 * t21052;
    let t21106 = -2.0 * t1657 * t19500 - t1772 * t21104 - t1775 * t21061 - 4.0 * t18496 * t21075 + 4.0 * t19509 * t6263 + 2.0 * t19509 * t6268 + t21053 * t538 - 6.0 * t21070 * t5739 + 4.0 * t21079 * t5739 + 2.0 * t21083 * t5739 - 2.0 * t21088 * t5739 + 2.0 * t21093 * t5739 + t21098 * t5739 + t21101 * t5739 + 2.0 * t5433 * t5734 - t5449 * t5734 - 2.0 * t6260 * t6271;
    (t21088, t21093, t21098, t21101, t21104, t21106)
}
