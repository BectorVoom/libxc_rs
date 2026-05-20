//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2076;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta605<F: Float>(t6790: F, t82632: F, t6787: F, t225: F, t23547: F, t23631: F, t974: F, t976: F, t984: F, t1009: F, t343: F, t25490: F, t6746: F, t884: F, t23384: F, t23715: F, t210: F, t23632: F, t23668: F, t23628: F, t6680: F, t23669: F, t995: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t82633, t82635, t82643, t82653, t82654, t82655) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2076::<F>(t6790, t82632, t6787, t225, t23547, t23631, t974, t976, t984, t1009, t343, t25490);
        let (t82657, t82661, t82668, t82694, t82713) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2077::<F>(t6746, t82655, t884, t23384, t23715, t210, t23632, t23668, t23628, t6680, t23669, t995);
    (t82633, t82635, t82643, t82653, t82654, t82655, t82657, t82661, t82668, t82694, t82713)
}
