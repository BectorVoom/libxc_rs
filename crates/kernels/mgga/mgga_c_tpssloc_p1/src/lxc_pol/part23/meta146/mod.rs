//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta146 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk688;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk689;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta146<F: Float>(t349: F, t5914: F, t1634: F, t3174: F, t381: F, t5872: F, t3188: F, t1615: F, t1625: F, t1060: F, t5866: F, t3201: F, t383: F, t1058: F, t1610: F, t1630: F, t1632: F, t3186: F, t3200: F, t353: F, t384: F, t4669: F, t5903: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5915, t5919, t5920, t5928, t5929, t5933, t5936, t5937, t5939) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk688::<F>(t349, t5914, t1634, t3174, t381, t5872, t3188, t1615, t1625, t1060, t5866, t3201);
        let (t5941, t5943) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk689::<F>(t383, t5914, t1058, t1610, t1630, t1632, t3186, t3200, t353, t384, t4669, t5903, t5929, t5933, t5937, t5939);
    (t5915, t5919, t5920, t5928, t5929, t5933, t5936, t5937, t5939, t5941, t5943)
}
