//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1179/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1179<F: Float>(t6553: F, t6554: F, t81984: F, t9458: F, t225: F, t23211: F, t23205: F, t82038: F, t23242: F, t81979: F, t10049: F, t10104: F, t1912: F, t218: F, t23191: F, t259: F, t2591: F, t2597: F, t40875: F, t6624: F, t6627: F, t6632: F, t6663: F, t81976: F, t866: F) -> (F,) {
    let t82282 = t81984 * t6553 * t6554 * t9458;
    let t82287 = t23211 * t225;
    let t82294 = t82038 * t23205;
    let t82296 = t81979 * t23242;
    let t82304 = -0.19739208802178717238e0 * t82282 - t6627 * t10104 + t218 * t81976 * t259 - 6.0 * t82287 * t866 + 6.0 * t10049 * t6632 - 3.0 * t2597 * t23191 - 0.15626873635058151147e0 * t82294 - 0.34543615403812755166e0 * t82296 + 3.0 * t2591 * t6624 * t259 - 3.0 * t10049 * t6663 - t40875 * t1912;
    (t82304,)
}
