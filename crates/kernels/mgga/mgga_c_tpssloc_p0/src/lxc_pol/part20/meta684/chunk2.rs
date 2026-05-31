//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2594/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2594<F: Float>(t1227: F, t13969: F, t15544: F, t15655: F, t15636: F, t3515: F, t1174: F, t44571: F, t4724: F, t11778: F, t43791: F, t11720: F, t11722: F, t11748: F, t15498: F, t3587: F, t44725: F, t44811: F, t44863: F, t45030: F, t4582: F, t48497: F, t4889: F, t4977: F, t52575: F) -> F {
    let t52580 = t1227 * t13969 * t15544;
    let t52583 = t1227 * t13969 * t15655;
    let t52586 = t3515 * t13969 * t15636;
    let t52599 = t1174 * t44571 * t4724;
    let t52600 = t52599 / F::cast_from(324.0_f64);
    let t52601 = t11778 * t43791;
    let t52606 = -F::cast_from(5.0_f64) / F::cast_from(864.0_f64) * t15498 * t3587 + t52575 / F::cast_from(108.0_f64) - t4889 * t11748 / F::cast_from(27.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t52580 + F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t52583 - t52586 / F::cast_from(768.0_f64) + t44863 * t4582 * t4977 * t44725 * t11720 / F::cast_from(128.0_f64) - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t45030 * t4582 * t4977 * t11722 + t44811 / F::cast_from(432.0_f64) - t52600 - F::cast_from(5.0_f64) / F::cast_from(432.0_f64) * t1227 * t4582 * t52601 * t48497;
    t52606
}
