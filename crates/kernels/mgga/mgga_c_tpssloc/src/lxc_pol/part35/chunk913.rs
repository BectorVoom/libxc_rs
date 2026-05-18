//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 913/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk913<F: Float>(t17763: F, t973: F, t2970: F, t5828: F, t10231: F, t5817: F, t2989: F, t5398: F, t2987: F, t5836: F, t5842: F, t13847: F, t4514: F) -> (F, F, F, F, F, F, F) {
    let t17764 = t973 * t17763;
    let t17769 = t2970 * t5828;
    let t17770 = t973 * t17769;
    let t17783 = t10231 * t5817;
    let t17784 = t973 * t17783;
    let t17794 = t2989 * t5398;
    let t17800 = t2987 * t5836;
    let t17804 = t2987 * t5842;
    let t17808 = t13847 * t4514;
    (t17764, t17770, t17784, t17794, t17800, t17804, t17808)
}
