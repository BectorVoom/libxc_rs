//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1676;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1677;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1678;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1679;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1680;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1681;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1682;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta392<F: Float>(t5980: F, t690: F, t3242: F, t5398: F, t607: F, t3240: F, t123: F, t3247: F, t1088: F, t1089: F, t16558: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t18229 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1676::<F>(t5980, t690);
        let (t18231, t18232) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1677::<F>(t3242, t5398, t607);
        let (t18233, t18234) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1678::<F>(t18232, t3240, t123);
        let (t18236, t18237) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1679::<F>(t3247, t5398, t607);
        let (t18238, t18239) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1680::<F>(t1088, t18237, t123);
        let t18241 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1681::<F>(t1089, t16558);
        let (t18242, t18243) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1682::<F>(t1088, t18241, t123);
    (t18229, t18231, t18232, t18233, t18234, t18236, t18237, t18238, t18239, t18241, t18242, t18243)
}
