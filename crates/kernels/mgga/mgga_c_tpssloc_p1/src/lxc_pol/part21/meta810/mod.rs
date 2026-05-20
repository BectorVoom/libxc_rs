//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta810 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2835;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2836;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2837;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2838;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2839;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2840;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2841;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta810<F: Float>(t2775: F, t55723: F, t123: F, t882: F, t2394: F, t5686: F, t17182: F, t2250: F, t17158: F, t690: F, t17162: F, t17153: F, t17151: F, t10564: F, t2244: F, t41666: F, t5392: F, t41664: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t59690, t59692) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2835::<F>(t2775, t55723, t123, t882);
        let t59694 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2836::<F>(t2394, t5686);
        let (t59696, t59698) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2837::<F>(t17182, t2250, t123, t882);
        let t59700 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2838::<F>(t17158, t690);
        let t59702 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2839::<F>(t17162, t690);
        let t59704 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2840::<F>(t17153, t690);
        let (t59706, t59708) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2841::<F>(t17151, t2250, t10564, t123);
        let (t59711, t59713) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2842::<F>(t2244, t41666, t5392, t123, t41664);
    (t59690, t59692, t59694, t59696, t59698, t59700, t59702, t59704, t59706, t59708, t59711, t59713)
}
