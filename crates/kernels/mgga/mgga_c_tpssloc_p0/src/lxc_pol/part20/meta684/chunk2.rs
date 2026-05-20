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
    let t52600 = t52599 / F::new(324.0);
    let t52601 = t11778 * t43791;
    let t52606 = -F::new(5.0) / F::new(864.0) * t15498 * t3587 + t52575 / F::new(108.0) - t4889 * t11748 / F::new(27.0) + F::new(5.0) / F::new(6912.0) * t52580 + F::new(5.0) / F::new(1152.0) * t52583 - t52586 / F::new(768.0) + t44863 * t4582 * t4977 * t44725 * t11720 / F::new(128.0) - F::new(3.0) / F::new(256.0) * t45030 * t4582 * t4977 * t11722 + t44811 / F::new(432.0) - t52600 - F::new(5.0) / F::new(432.0) * t1227 * t4582 * t52601 * t48497;
    t52606
}
