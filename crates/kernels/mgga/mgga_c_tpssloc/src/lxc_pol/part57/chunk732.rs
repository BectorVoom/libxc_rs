//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 732/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk732<F: Float>(t1860: F, t2032: F, t23963: F, t23995: F, t26016: F, t26911: F, t26920: F, t26936: F, t26948: F, t26954: F, t26960: F, t27937: F, t27961: F, t27966: F, t27972: F, t27976: F, t27979: F, t27982: F, t28935: F, t7026: F, t7428: F, t7432: F, t7435: F, t7782: F) -> (F,) {
    let t28941 = t27937 * t2032 / 3.0 + 2.0 / 3.0 * t7428 * t7782 + 10.0 * t23963 * t27961 + 20.0 / 3.0 * t26016 * t26954 + t23995 - 10.0 / 3.0 * t7026 * t27972 - 5.0 / 3.0 * t7026 * t27976 - 2.0 / 3.0 * t27979 * t2032 - 2.0 / 3.0 * t27982 * t2032 - 4.0 / 3.0 * t7435 * t7782 - 16.0 / 9.0 * t26948 - 10.0 / 3.0 * t26911 * t7432 - 4.0 / 3.0 * t27966 * t2032 + t1860 * t28935 / 3.0 + 80.0 / 9.0 * t26920 - 16.0 / 9.0 * t26960 + 32.0 / 9.0 * t26936;
    (t28941,)
}
