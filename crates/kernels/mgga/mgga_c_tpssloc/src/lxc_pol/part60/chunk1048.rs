//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1048/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1048<F: Float>(t27254: F, t7467: F, t100996: F, t1873: F, t2113: F, t5493: F, t1458: F, t7982: F, t2240: F, t29473: F, t8301: F, t55921: F, t8662: F) -> (F, F, F, F, F, F) {
    let t128984 = F::new(27.0) * t27254 * t7467;
    let t128988 = F::new(0.135e2) * t100996 * t1873;
    let t129008 = t2113 * t5493;
    let t129015 = t7982 * t1458;
    let t129084 = t2240 * t8301 * t29473;
    let t129093 = t55921 * t8662;
    (t128984, t128988, t129008, t129015, t129084, t129093)
}
