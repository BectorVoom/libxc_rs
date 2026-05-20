//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2088;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta487<F: Float>(t5544: F, t845: F, t776: F, t16662: F, t824: F, t1504: F, t1506: F, t16723: F, t16729: F, t16737: F, t16740: F, t228: F, t230: F, t4219: F, t4225: F, t4227: F, t4230: F, t5601: F, t5605: F, t5608: F, t822: F, t825: F, t232: F, t860: F, t2732: F, t5612: F, t1509: F, t1519: F) -> (F, F, F, F, F, F, F, F) {
        let (t16745, t16746, t16749, t16752) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2088::<F>(t5544, t845, t776, t16662, t824, t1504, t1506, t16723, t16729, t16737, t16740, t228, t230, t4219, t4225, t4227, t4230, t5601, t5605, t5608, t822, t825);
        let (t16753, t16754, t16756, t16758) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2089::<F>(t16752, t232, t860, t2732, t5612, t1509, t1519);
    (t16745, t16746, t16749, t16752, t16753, t16754, t16756, t16758)
}
