//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 861/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk861<F: Float>(t6883: F, t8612: F, t532: F, t8639: F, t8662: F, t9239: F, t131: F, t7245: F, t2240: F, t31: F, t63: F, t79: F) -> (F, F, F, F, F, F, F) {
    let t31662 = t6883 * t8612;
    let t31663 = F::new(0.19190897446562641759e-1) * t31662;
    let t31758 = t532 * t8639;
    let t31860 = t9239 * t8662;
    let t31863 = t7245 * t131;
    let t31864 = t2240 * t31863;
    let t32331 = t63 * t31;
    let t32338 = t79 * t63;
    (t31663, t31758, t31860, t31863, t31864, t32331, t32338)
}
