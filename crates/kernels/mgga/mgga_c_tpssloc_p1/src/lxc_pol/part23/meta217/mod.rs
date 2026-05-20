//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta217 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk863;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta217<F: Float>(t3069: F, t4669: F, t1612: F, t3082: F, t1606: F, t698: F, t973: F, t1043: F, t2770: F, t10277: F, t3061: F, t10216: F, t10969: F, t10868: F, t1539: F, t248: F, t1041: F, t1615: F, t3131: F, t360: F, t883: F, t1573: F, t2904: F, t1561: F, t2885: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13995, t14117, t14160, t14164, t14172, t14187) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk863::<F>(t3069, t4669, t1612, t3082, t1606, t698, t973, t1043, t2770, t10277, t3061, t10216, t10969);
        let (t14202, t14203, t14211, t14219, t14263, t14271) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk864::<F>(t10868, t1539, t248, t1041, t1615, t3131, t360, t883, t1573, t2904, t1561, t2885);
    (t13995, t14117, t14160, t14164, t14172, t14187, t14202, t14203, t14211, t14219, t14263, t14271)
}
