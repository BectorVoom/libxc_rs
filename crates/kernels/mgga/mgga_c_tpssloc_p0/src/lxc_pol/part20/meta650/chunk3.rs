//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2393/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2393<F: Float>(t10704: F, t4395: F, t10702: F, t2793: F, t10524: F, t10603: F, t10717: F, t10724: F, t10734: F, t10756: F, t10765: F, t14271: F, t14276: F, t14337: F, t14369: F, t14459: F, t14466: F, t1580: F, t2906: F, t2924: F, t2930: F, t41826: F, t41981: F, t42111: F, t42113: F, t42123: F, t4416: F, t4438: F, t4475: F, t48883: F, t48890: F, t49068: F, t49071: F, t950: F) -> (F, F) {
    let t49072 = t4395 * t10704;
    let t49075 = F::cast_from(0.1551780387578202009e4_f64) * t10702 * t49072 * t2793;
    let t49076 = -F::cast_from(6.0_f64) * t14276 * t10734 + F::cast_from(0.96491876992155210402e2_f64) * t14271 * t10717 - F::cast_from(6.0_f64) * t41981 * t4416 + F::cast_from(0.96491876992155210402e2_f64) * t42123 * t4438 + F::cast_from(0.17315859105681463759e2_f64) * t2930 * t4475 * t10603 + F::cast_from(0.91082604192152556044e5_f64) * t42111 * t1580 * t42113 * t10524 + F::cast_from(0.51947577317044391277e2_f64) * t14337 * t10724 - F::cast_from(0.12304822629859687989e5_f64) * t41826 * t14369 * t10524 + F::cast_from(0.51947577317044391277e2_f64) * t2930 * t48883 * t950 + F::cast_from(0.51947577317044391277e2_f64) * t2930 * t14459 * t2924 + F::cast_from(0.30762056574649219973e4_f64) * t10756 * t48890 * t2906 + F::cast_from(18.0_f64) * t10765 * t14466 - t49068 - t49071 - t49075;
    (t49075, t49076)
}
