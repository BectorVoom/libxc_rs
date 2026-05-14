//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 972/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk972<F: Float>(t2987: F, t5836: F, t2990: F, t5842: F, t13847: F, t4514: F, t2986: F, t17167: F, t4518: F, t17171: F, t10254: F, t5392: F, t2988: F, t17183: F, t135: F, t5844: F) -> (F, F, F, F, F, F, F, F) {
    let t17800 = t2987 * t5836;
    let t17801 = t17800 * t2990;
    let t17804 = t2987 * t5842;
    let t17805 = t17804 * t2990;
    let t17808 = t13847 * t4514;
    let t17809 = t2986 * t17808;
    let t17811 = t4518 * t17167;
    let t17814 = t4518 * t17171;
    let t17817 = t10254 * t5392;
    let t17818 = t2988 * t17817;
    let t17821 = t4518 * t17183;
    let t17826 = t135 * t5844;
    (t17801, t17805, t17809, t17811, t17814, t17818, t17821, t17826)
}
