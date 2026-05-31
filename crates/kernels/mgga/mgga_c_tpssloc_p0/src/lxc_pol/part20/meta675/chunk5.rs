//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2549/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2549<F: Float>(t11275: F, t1670: F, t1117: F, t43976: F, t11285: F, t4857: F, t11129: F, t11303: F, t11310: F, t11365: F, t11399: F, t11437: F, t11441: F, t1155: F, t15133: F, t15146: F, t15153: F, t15207: F, t15218: F, t15225: F, t1694: F, t1695: F, t3376: F, t3377: F, t3395: F, t3401: F, t43692: F, t44155: F, t44223: F, t4858: F, t4861: F) -> (F, F) {
    let t51638 = t11275 * t1670;
    let t51641 = F::cast_from(0.1551780387578202009e4_f64) * t51638 * t43976 * t1117;
    let t51651 = t4857 * t11285;
    let t51664 = -F::cast_from(0.35089341735807877242e1_f64) * t3376 * t15133 * t1155 - F::cast_from(0.35089341735807877242e1_f64) * t3376 * t4858 * t3395 - F::cast_from(0.31168546390226634765e3_f64) * t11365 * t15218 * t3377 - F::cast_from(0.11696447245269292414e1_f64) * t3376 * t1695 * t11399 - F::cast_from(0.12304822629859687989e5_f64) * t44155 * t15225 * t11129 - F::cast_from(6.0_f64) * t15207 * t11437 - F::cast_from(12.0_f64) * t11303 * t15153 - t51641 - F::cast_from(0.14035736694323150897e2_f64) * t11365 * t1695 * t11129 + F::cast_from(0.10526802520742363173e2_f64) * t3401 * t4858 * t3377 + F::cast_from(0.51947577317044391277e2_f64) * t3401 * t15218 * t3395 + F::cast_from(0.30762056574649219973e4_f64) * t11310 * t51651 * t3377 + F::cast_from(0.17315859105681463759e2_f64) * t3401 * t4861 * t11399 + F::cast_from(0.91082604192152556044e5_f64) * t44223 * t1694 * t43692 * t11129 + F::cast_from(0.96491876992155210402e2_f64) * t15146 * t11441;
    (t51641, t51664)
}
