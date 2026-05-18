//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 750/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk750<F: Float>(t5399: F, t605: F, t1860: F, t1865: F, t22544: F, t26013: F, t26016: F, t26051: F, t26084: F, t27937: F, t27950: F, t27953: F, t27957: F, t27961: F, t27966: F, t27972: F, t27976: F, t27979: F, t6490: F, t7428: F, t7432: F, t7435: F, t7442: F, t7446: F) -> (F, F) {
    let t27982 = t605 * t5399;
    let t27991 = -t27937 * t1865 / F::new(6.0) - t7428 * t7442 / F::new(3.0) - t7428 * t7446 / F::new(3.0) - t1860 * t27950 / F::new(6.0) - t1860 * t27953 / F::new(3.0) - t1860 * t27957 / F::new(6.0) - F::new(5.0) * t22544 * t27961 - F::new(10.0) / F::new(3.0) * t26016 * t26013 + F::new(2.0) / F::new(3.0) * t27966 * t1865 + F::new(5.0) / F::new(3.0) * t26084 * t7432 + F::new(5.0) / F::new(3.0) * t6490 * t27972 + F::new(5.0) / F::new(6.0) * t6490 * t27976 + t27979 * t1865 / F::new(3.0) + t27982 * t1865 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t7435 * t7442 + F::new(2.0) / F::new(3.0) * t7435 * t7446 + F::new(5.0) / F::new(3.0) * t26051 * t7432;
    (t27982, t27991)
}
