//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2406/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2406<F: Float>(t10828: F, t1580: F, t10524: F, t10724: F, t10740: F, t10743: F, t10771: F, t10811: F, t10825: F, t14329: F, t14425: F, t14429: F, t14435: F, t14463: F, t1581: F, t2861: F, t2862: F, t2880: F, t4434: F, t4437: F, t49222: F, t49228: F, t49244: F, t49256: F, t49259: F, t49262: F, t931: F, t943: F, t951: F) -> F {
    let t49263 = t10828 * t1580;
    let t49266 = F::cast_from(0.5848223622634646207e0_f64) * t943 * t49222 * t951 - t49228 - F::cast_from(12.0_f64) * t10740 * t14425 - F::cast_from(6.0_f64) * t2861 * t14329 * t931 - F::cast_from(6.0_f64) * t2861 * t4434 * t2880 - F::cast_from(0.57895126195293126242e3_f64) * t10771 * t14435 * t2862 - t49244 - F::cast_from(6.0_f64) * t10740 * t14429 - F::cast_from(0.14035736694323150897e2_f64) * t10828 * t1581 * t10524 + F::cast_from(0.11579025239058625248e4_f64) * t10811 * t4437 * t10743 + F::cast_from(0.10526802520742363173e2_f64) * t10825 * t14463 - t49256 - t49259 - t49262 - F::cast_from(0.31168546390226634766e3_f64) * t49263 * t10724;
    t49266
}
