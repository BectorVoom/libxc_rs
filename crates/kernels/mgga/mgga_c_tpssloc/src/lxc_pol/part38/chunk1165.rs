//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1165/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1165<F: Float>(t29922: F, t659: F, t2341: F, t91: F, t2342: F, t2248: F, t8138: F, t29894: F, t29896: F, t29898: F, t29901: F, t29903: F, t29904: F, t29908: F, t29912: F, t29915: F, t29919: F, t64: F, t8128: F, t8137: F) -> (F, F, F, F, F) {
    let t29923 = t29922 * t659;
    let t29926 = t91 * t2341;
    let t29927 = t29926 * t2342;
    let t29930 = t8138 * t2248;
    let t29933 = -t29894 - 4.0 / 3.0 * t29896 - 10.0 / 9.0 * t29898 + 10.0 / 9.0 * t29901 - 3.0 / 4.0 * t29903 * t29904 - 5.0 / 6.0 * t8128 * t29908 + 5.0 / 6.0 * t8128 * t29912 + t8128 * t29915 / 4.0 - 5.0 / 9.0 * t64 * t29919 + 25.0 / 36.0 * t8137 * t29923 - 5.0 / 36.0 * t8137 * t29927 - 5.0 / 24.0 * t8137 * t29930;
    (t29923, t29926, t29927, t29930, t29933)
}
