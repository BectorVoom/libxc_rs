//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1086/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1086<F: Float>(t11568: F, t14973: F, t2741: F, t242: F, t2675: F, t4989: F, t946: F, t140: F, t4965: F, t925: F, t4984: F, t8983: F) -> (F, F, F, F) {
    let t14974 = t11568 * t14973;
    let t14975 = t2741 * t14974;
    let t14979 = t242 * t2675 * t4989;
    let t14980 = t946 * t14979;
    let t14986 = t140 * t4965;
    let t14987 = t925 * t14986;
    let t14991 = t8983 * t4984;
    (t14975, t14980, t14987, t14991)
}
