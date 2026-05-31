//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 217/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk217<F: Float>(t109: F, t659: F, t95: F, t103: F, t100: F, t657: F, t92: F, t96: F, t656: F, t64: F, t654: F) -> (F, F, F, F, F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t660 = t95 * t659;
    let t662 = -t659;
    let t663 = t103 * t662;
    let t666 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t100 * t663 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t657 * t96 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t92 * t660;
    let t667 = t656 * t666;
    let t671 = piecewise3::<F>(t110, F::cast_from(0.0_f64), -t654 - t64 * t667 / F::cast_from(8.0_f64));
    (t660, t662, t663, t666, t667, t671)
}
