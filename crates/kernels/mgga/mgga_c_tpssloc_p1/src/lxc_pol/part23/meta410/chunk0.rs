//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1225/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1225<F: Float>(t11789: F, t1227: F, t248: F, t5975: F, t15437: F, t15502: F, t15506: F, t19201: F, t3576: F, t3577: F, t44951: F, t6191: F) -> (F, F, F, F, F) {
    let t65689 = t1227 * t248 * t11789 * t5975;
    let t65703 = t15437 * t15502;
    let t65706 = t15437 * t15506;
    let t65815 = t19201 * t3576;
    let t65819 = t3577 * t44951 * t6191;
    (t65689, t65703, t65706, t65815, t65819)
}
