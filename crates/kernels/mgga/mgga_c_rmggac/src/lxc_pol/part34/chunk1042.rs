//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1042/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1042<F: Float>(t76281: F, t76283: F, t76285: F, t76287: F, t76289: F, t2123: F, t9530: F, t118: F, t5259: F, t551: F, t71903: F, t76305: F) -> (F, F, F, F, F, F, F, F, F) {
    let t78017 = F::cast_from(0.81823984962736025184e-1_f64) * t76281;
    let t78018 = F::cast_from(0.20455996240684006296e-1_f64) * t76283;
    let t78019 = F::cast_from(0.81823984962736025184e-1_f64) * t76285;
    let t78020 = F::cast_from(0.20455996240684006296e0_f64) * t76287;
    let t78021 = F::cast_from(0.40911992481368012592e-1_f64) * t76289;
    let t78022 = t9530 * t2123;
    let t78024 = F::cast_from(0.39914139006212695214e-1_f64) * t118 * t78022;
    let t78026 = t5259 * t71903 * t551;
    let t78027 = F::cast_from(0.2993560425465952141e-1_f64) * t78026;
    let t78028 = F::cast_from(0.79828278012425390427e-1_f64) * t76305;
    (t78017, t78018, t78019, t78020, t78021, t78022, t78024, t78027, t78028)
}
