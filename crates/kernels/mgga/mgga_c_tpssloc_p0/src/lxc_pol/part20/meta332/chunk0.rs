//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1619/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1619<F: Float>(t1174: F, t11761: F, t11766: F, t11770: F, t11774: F, t11781: F, t11787: F, t11792: F, t11794: F, t11798: F, t11802: F, t11805: F, t11809: F, t11814: F, t1218: F, t1227: F, t3515: F) -> F {
    let t11817 = t1174 * t11761 / F::cast_from(36.0_f64) - F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1174 * t11766 - t3515 * t11770 / F::cast_from(1024.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t1227 * t11774 - F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t1227 * t11781 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t11787 + t11792 / F::cast_from(6912.0_f64) + t11794 / F::cast_from(768.0_f64) - t11798 / F::cast_from(2304.0_f64) - t11802 / F::cast_from(1152.0_f64) - t1227 * t11805 / F::cast_from(4608.0_f64) - t1227 * t11809 / F::cast_from(768.0_f64) + t11814 * t1218 / F::cast_from(1024.0_f64);
    t11817
}
