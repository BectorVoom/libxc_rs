//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1225;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta410<F: Float>(t11789: F, t1227: F, t248: F, t5975: F, t15437: F, t15502: F, t15506: F, t19201: F, t3576: F, t3577: F, t44951: F, t6191: F, t15568: F, t5064: F, t45046: F, t5971: F, t3032: F, t65253: F, t3505: F, t3514: F, t1174: F, t6187: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t65689, t65703, t65706, t65815, t65819) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1225::<F>(t11789, t1227, t248, t5975, t15437, t15502, t15506, t19201, t3576, t3577, t44951, t6191);
        let (t65884, t65935, t65963, t65966, t66015) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1226::<F>(t15568, t5064, t1227, t248, t45046, t5971, t3032, t65253, t3505, t3514, t1174, t6187, t698);
    (t65689, t65703, t65706, t65815, t65819, t65884, t65935, t65963, t65966, t66015)
}
