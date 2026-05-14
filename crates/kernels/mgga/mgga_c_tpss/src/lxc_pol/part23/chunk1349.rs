//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1349/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1349<F: Float>(t67904: F, t68645: F, t68657: F, t68681: F, t68695: F, t68701: F, t68728: F, t68742: F, t3410: F, t4559: F, t548: F, t6067: F, t63699: F, t63701: F, t63703: F, t63705: F, t6552: F, t66068: F, t66073: F, t66075: F, t66077: F, t66080: F, t66083: F, t66087: F, t66091: F, t66094: F, t66098: F) -> (F, F) {
    let t68745 = t67904 + t68645 + t68657 + t68681 + t68695 + t68701 + t68728 + t68742;
    let t68752 = t548 * t68745 * param_d + 3.0 * t3410 * t6552 + 6.0 * t4559 * t6067 + t63699 + t63701 + t63703 + t63705 + t66068 + t66073 + t66075 + t66077 + t66080 + t66083 + t66087 + t66091 + t66094 + t66098;
    (t68745, t68752)
}
