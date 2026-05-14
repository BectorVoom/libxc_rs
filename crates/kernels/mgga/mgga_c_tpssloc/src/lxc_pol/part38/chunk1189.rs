//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1189/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1189<F: Float>(t110736: F, t110778: F, t110826: F, t110870: F, t110671: F, t110684: F, t12524: F, t12813: F, t1458: F, t16521: F, t16524: F, t16538: F, t16541: F, t2180: F, t2363: F, t29934: F, t29993: F, t29996: F, t30012: F, t30180: F, t30231: F, t30253: F, t30258: F, t3941: F, t4072: F, t55341: F, t55353: F, t55571: F, t577: F, t671: F, t8143: F, t8166: F, t8230: F, t8251: F) -> (F, F) {
    let t110872 = t110736 + t110778 + t110826 + t110870;
    let t110877 = 27.0 * t29996 * t16541 + 54.0 * t29996 * t16538 + 54.0 * t12524 * t30258 + 27.0 * t16524 * t30012 + 54.0 * t12524 * t30253 + 27.0 * t3941 * t29934 * t1458 + 27.0 * t16521 * t8143 + 27.0 * t110671 * t1458 + 27.0 * t29993 * t4072 + 54.0 * t3941 * t30180 * t671 + 27.0 * t3941 * t8230 * t2363 + 0.135e2 * t30231 * t2363 + 27.0 * t110684 * t671 + 27.0 * t3941 * t2180 * t12813 + 27.0 * t55571 * t8251 + 0.135e2 * t55341 * t2180 + 0.45e1 * t110872 * t577 + 54.0 * t55353 * t8166;
    (t110872, t110877)
}
