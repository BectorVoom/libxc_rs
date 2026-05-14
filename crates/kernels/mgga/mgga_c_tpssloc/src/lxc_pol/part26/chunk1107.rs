//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1107/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1107<F: Float>(t213: F, t80893: F, t12156: F, t1998: F, t236: F, t12328: F, t2003: F, t12248: F, t59: F, t1336: F, t240: F, t12293: F, t12297: F, t22761: F, t12305: F, t6952: F) -> (F, F, F, F, F) {
    let t80894 = t80893 * t213;
    let t80897 = t80894 * t1998 * t236 * t12156;
    let t80899 = t2003 * t12328;
    let t80900 = 595.0 / 5184.0 * t80899;
    let t80901 = t12248 * t59;
    let t80903 = t1336 * t80901 * t240;
    let t80904 = t80903 * t12293;
    let t80906 = t22761 * t12297;
    let t80908 = t6952 * t12305;
    (t80897, t80900, t80904, t80906, t80908)
}
