//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta802 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2789;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta802<F: Float>(t12940: F, t58994: F, t12606: F, t4194: F, t4195: F, t12908: F, t16713: F, t12939: F, t5392: F, t607: F, t750: F, t157: F, t4196: F, t46447: F, t41274: F, t39658: F, t41254: F, t41258: F, t41262: F, t58983: F, t58985: F, t58986: F, t58987: F, t58988: F, t58989: F, t58990: F, t58991: F, t58993: F) -> (F, F, F, F, F, F, F) {
        let (t58996, t58999, t59001, t59005, t59008) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2789::<F>(t12940, t58994, t12606, t4194, t4195, t12908, t16713, t12939, t5392, t607, t750, t157, t4196, t46447);
        let (t59009, t59010) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2790::<F>(t41274, t39658, t41254, t41258, t41262, t58983, t58985, t58986, t58987, t58988, t58989, t58990, t58991, t58993, t58996, t58999, t59001, t59005, t59008);
    (t58996, t58999, t59001, t59005, t59008, t59009, t59010)
}
