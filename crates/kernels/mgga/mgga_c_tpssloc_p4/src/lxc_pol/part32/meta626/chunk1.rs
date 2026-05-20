//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2036/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2036<F: Float>(t86967: F, t1484: F, t2717: F, t225: F, t25051: F, t23012: F, t7489: F, t23164: F, t23204: F, t25341: F, t1887: F, t81956: F) -> (F, F, F, F, F, F) {
    let t86968 = F::cast_from(0.76763589786250567036e-1_f64) * t86967;
    let t86969 = t2717 * t1484;
    let t86988 = t25051 * t225;
    let t86991 = t23012 * t7489;
    let t87028 = t23164 * t23204 * t25341;
    let t87029 = F::cast_from(0.16449340668482264365e-1_f64) * t87028;
    let t87049 = t81956 * t1887;
    (t86968, t86969, t86988, t86991, t87029, t87049)
}
