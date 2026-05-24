//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 931/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk931<F: Float>(t2719: F, t956: F, t2713: F, t2716: F, t941: F, t2751: F, t774: F, t2742: F, t2740: F, t348: F, t2738: F, t983: F) -> (F, F, F, F, F) {
    let t8970 = t956 * t2719;
    let t8972 = t2713 * t2716 * t8970;
    let t8976 = t2713 * t941 * t8970;
    let t8983 = t774 * t2751;
    let t8984 = t8983 * t2742;
    let t8985 = t2740 * t8984;
    let t8987 = t348 * t956;
    let t8989 = t983 * t8987 * t2738;
    (t8972, t8976, t8983, t8985, t8989)
}
