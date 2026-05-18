//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1305/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1305<F: Float>(t1275: F, t5960: F, t19010: F, t550: F, t116: F, t18627: F, t1856: F, t3398: F, t1848: F, t3413: F, t1284: F, t5941: F) -> (F, F, F, F, F, F) {
    let t63114 = t1275 * t5960;
    let t63116 = t19010 * t550;
    let t63152 = t116 * t18627;
    let t63167 = t3398 * t1856;
    let t63169 = t1848 * t3413;
    let t63173 = t5941 * t1284;
    (t63114, t63116, t63152, t63167, t63169, t63173)
}
