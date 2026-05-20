//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1556/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1556<F: Float>(t14228: F, t4337: F, t10408: F, t13510: F, t13512: F, t13514: F, t13517: F, t13519: F, t13522: F, t13524: F, t13526: F, t13657: F, t13661: F, t13665: F, t13720: F, t13722: F, t13726: F, t13729: F, t13731: F, t13734: F) -> (F, F) {
    let t14234 = t4337 * t14228;
    let t14235 = t10408 * t14234;
    let t14238 = -t13510 + t13512 - t13514 + t13517 + t13519 + t13522 + t13524 + t13526 + t13657 - t13661 + t13665 - t13720 + t13722 + t13726 - t13729 - t13731 + t13734;
    (t14235, t14238)
}
