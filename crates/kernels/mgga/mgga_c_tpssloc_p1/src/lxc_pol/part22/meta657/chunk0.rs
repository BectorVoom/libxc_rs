//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2199/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2199<F: Float>(t58181: F, t816: F, t16969: F, t9638: F, t13258: F, t16928: F, t41385: F, t5587: F, t842: F, t16673: F, t2696: F, t849: F) -> (F, F, F, F, F, F, F) {
    let t58765 = t58181 * t816;
    let t58791 = t9638 * t16969;
    let t58797 = t13258 * t16928;
    let t58809 = t41385 * t5587;
    let t58834 = t58181 * t842;
    let t58844 = t16673 * t2696;
    let t58845 = t58844 * t849;
    (t58765, t58791, t58797, t58809, t58834, t58844, t58845)
}
