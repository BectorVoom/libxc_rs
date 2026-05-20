//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1757/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1757<F: Float>(t24658: F, t7325: F, t3030: F, t3502: F, t478: F, t1209: F, t1222: F, t7334: F, t2141: F, t3540: F, t3: F, t7324: F) -> (F, F, F, F, F, F, F, F) {
    let t24659 = t24658 * t7325;
    let t24660 = t3030 * t3502;
    let t24661 = t24660 * t478;
    let t24667 = t3030 * t1209;
    let t24668 = t24667 * t478;
    let t24675 = t7334 * t1222;
    let t24681 = t2141 * t3540 / F::new(6912.0);
    let t24682 = t7324 * t3;
    (t24659, t24660, t24661, t24667, t24668, t24675, t24681, t24682)
}
