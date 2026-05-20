//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2193/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2193<F: Float>(t13133: F, t4101: F, t16616: F, t2371: F, t17083: F, t225: F, t16805: F, t68: F, t16752: F, t252: F, t13396: F, t1499: F) -> (F, F, F, F, F, F) {
    let t58052 = t13133 * t4101;
    let t58057 = t16616 * t2371;
    let t58143 = t17083 * t225;
    let t58181 = t16805 * t68;
    let t58262 = t252 * t16752;
    let t58313 = t1499 * t13396;
    (t58052, t58057, t58143, t58181, t58262, t58313)
}
