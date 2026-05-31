//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 611/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk611<F: Float>(t7240: F, t7252: F, t7256: F, t7260: F, t2212: F, t4965: F, t2265: F, t931: F, t7266: F, t7276: F, t7285: F, t4616: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8025 = F::cast_from(0.638468998399467591e-4_f64) * t7240;
    let t8027 = F::cast_from(0.5107751987195740728e-4_f64) * t7252;
    let t8028 = F::cast_from(0.1702583995731913576e-4_f64) * t7256;
    let t8029 = F::cast_from(0.85129199786595678799e-5_f64) * t7260;
    let t8030 = t4965 * t2212;
    let t8031 = F::cast_from(0.79828278012425390428e-1_f64) * t8030;
    let t8033 = t931 * t2265;
    let t8034 = F::cast_from(0.2363e1_f64) * t8033;
    let t8035 = F::cast_from(0.13637330827122670865e-1_f64) * t7266;
    let t8037 = F::cast_from(0.13637330827122670865e0_f64) * t7276;
    let t8039 = F::cast_from(0.40911992481368012596e-1_f64) * t7285;
    let t8041 = t4616 * t698;
    (t8025, t8027, t8028, t8029, t8031, t8034, t8035, t8037, t8039, t8041)
}
