//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1018/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1018<F: Float>(t6612: F, t812: F, t836: F, t2649: F, t10003: F, t23146: F, t10009: F, t25084: F, t9629: F, t9623: F, t23127: F, t2707: F, t2690: F, t6619: F, t849: F, t6620: F, t9612: F) -> (F, F, F, F, F, F, F, F) {
    let t81749 = t812 * t6612 * t836;
    let t81750 = t81749 * t2649;
    let t81752 = t23146 * t10003;
    let t81754 = t23146 * t10009;
    let t81756 = t25084 * t9629;
    let t81758 = t23146 * t9623;
    let t81760 = t23127 * t2707;
    let t81763 = t812 * t6619 * t2690;
    let t81764 = t81763 * t849;
    let t81766 = t9612 * t6620;
    (t81750, t81752, t81754, t81756, t81758, t81760, t81764, t81766)
}
