//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta841 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3025;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3026;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3027;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3028;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3029;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3030;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3031;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3032;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3033;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta841<F: Float>(t18233: F, t690: F, t18207: F, t16558: F, t3242: F, t607: F, t123: F, t3240: F, t18231: F, t2250: F, t47774: F, t51002: F, t55716: F, t50826: F, t50828: F, t50834: F, t63291: F, t63296: F, t63300: F, t63304: F, t50998: F, t50992: F, t2394: F, t5972: F, t5980: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t63306 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3025::<F>(t18233, t690);
        let t63308 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3026::<F>(t18207, t690);
        let (t63311, t63313) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3027::<F>(t16558, t3242, t607, t123, t3240);
        let (t63315, t63317) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3028::<F>(t18231, t2250, t123, t3240);
        let t63323 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3029::<F>(t47774, t51002, t55716);
        let t63325 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3030::<F>(t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317, t63323);
        let t63327 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3031::<F>(t47774, t50998, t55716);
        let t63330 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3032::<F>(t47774, t50992, t55716);
        let t63332 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3033::<F>(t2394, t5972);
        let t63334 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3034::<F>(t2394, t5980);
    (t63306, t63308, t63311, t63313, t63315, t63317, t63323, t63325, t63327, t63330, t63332, t63334)
}
