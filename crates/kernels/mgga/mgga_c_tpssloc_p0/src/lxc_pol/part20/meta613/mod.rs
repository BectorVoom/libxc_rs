//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2201;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2202;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta613<F: Float>(t35761: F, t35577: F, t112: F, t12512: F, t111: F, t3931: F, t16546: F, t576: F, t16506: F, t580: F, t2319: F, t4025: F, t2311: F, t671: F, t11968: F, t1266: F, t12724: F, t12728: F, t12835: F, t12841: F, t1442: F, t1459: F, t15857: F, t1774: F, t2312: F, t3652: F, t4026: F, t4034: F, t4037: F, t510: F, t5107: F, t650: F, t9347: F, t9348: F, t9351: F, t12723: F, t2363: F, t649: F, t89: F, t9416: F, t12492: F, t12557: F, t12725: F, t12734: F, t12813: F, t12816: F, t12823: F, t1393: F, t1458: F, t1778: F, t1849: F, t19456: F, t2314: F, t2364: F, t652: F, t672: F, t9419: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t45460, t45496, t45557, t45560, t45584, t45588, t45590) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2201::<F>(t35761, t35577, t112, t12512, t111, t3931, t16546, t576, t16506, t580, t2319, t4025);
        let (t45602, t45616) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2202::<F>(t2311, t671, t11968, t1266, t12724, t12728, t12835, t12841, t1442, t1459, t15857, t1774, t2312, t3652, t4026, t4034, t4037, t45590, t510, t5107, t650, t9347, t9348, t9351);
        let (t45632, t45637, t45648) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2203::<F>(t111, t12723, t2363, t649, t89, t9416, t11968, t12492, t12557, t1266, t12725, t12734, t12813, t12816, t12823, t12835, t1393, t1458, t1459, t1778, t1849, t19456, t2314, t2364, t4037, t652, t672, t9419);
    (t45460, t45496, t45557, t45560, t45584, t45588, t45590, t45602, t45616, t45632, t45637, t45648)
}
