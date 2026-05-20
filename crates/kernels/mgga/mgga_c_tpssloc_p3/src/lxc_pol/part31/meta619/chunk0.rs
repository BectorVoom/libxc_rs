//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1869/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1869<F: Float>(t1307: F, t26331: F, t26446: F, t96951: F, t1992: F, t550: F, t57545: F, t6976: F, t19893: F, t90914: F, t90915: F, t1799: F, t1834: F) -> (F, F, F, F) {
    let t96954 = t26331 * t26446 * t96951 * t1307;
    let t96958 = t1992 * t6976 * t57545 * t550;
    let t96962 = t90914 * t90915 * t19893;
    let t96964 = t1834 * t1799;
    (t96954, t96958, t96962, t96964)
}
