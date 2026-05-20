//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1843/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1843<F: Float>(t776: F, t865: F, t22986: F, t23270: F, t25044: F, t13377: F, t1880: F, t214: F, t225: F, t258: F, t1887: F, t81956: F) -> (F, F, F, F) {
    let t87036 = t776 * t865;
    let t87039 = t22986 * t23270 * t25044 * t87036;
    let t87047 = t1880 * t214 * t13377 * t225 * t258;
    let t87049 = t81956 * t1887;
    (t87036, t87039, t87047, t87049)
}
