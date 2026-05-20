//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1363/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1363<F: Float>(t756: F, t9874: F, t9727: F, t9780: F, t9789: F, t9793: F, t9797: F, t9863: F, t9865: F, t9867: F, t9870: F, t9872: F) -> (F, F) {
    let t9876 = F::cast_from(0.56968947174242584612e-3_f64) * t756 * t9874;
    let t9877 = t9727 + t9863 + t9780 + t9865 - t9867 - t9789 + t9870 + t9872 + t9793 + t9797 - t9876;
    (t9876, t9877)
}
