//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1022/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1022<F: Float>(t13242: F, t4180: F, t4182: F, t4181: F, t9632: F, t2642: F, t4166: F, t2617: F, t4177: F, t2628: F, t836: F, t812: F) -> (F, F, F, F, F) {
    let t13244 = t4180 * t13242 * t4182;
    let t13248 = t4180 * t4181 * t9632;
    let t13251 = t4166 * t2642;
    let t13254 = t2617 * t4177;
    let t13257 = t2628 * t836;
    let t13258 = t812 * t13257;
    (t13244, t13248, t13251, t13254, t13258)
}
