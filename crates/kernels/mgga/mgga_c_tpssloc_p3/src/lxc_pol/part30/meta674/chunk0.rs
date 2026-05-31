//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2103/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2103<F: Float>(t90980: F, t90993: F, t91000: F, t91149: F, t91167: F, t91305: F, t91312: F, t91394: F, t91398: F, t91078: F, t91081: F, t91531: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t93595 = F::cast_from(0.16449340668482264365e-1_f64) * t90980;
    let t93605 = F::cast_from(0.16449340668482264365e-1_f64) * t90993;
    let t93615 = F::cast_from(0.12793931631041761173e0_f64) * t91000;
    let t93650 = F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t91149;
    let t93656 = F::cast_from(0.22608743412718618878e-1_f64) * t91167;
    let t93721 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t91305;
    let t93723 = F::cast_from(0.10541775202358879834e-2_f64) * t91312;
    let t93757 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t91394;
    let t93760 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t91398;
    let t93795 = F::cast_from(0.52089578783527170489e-1_f64) * t91078;
    let t93796 = F::cast_from(0.3289868133696452873e-1_f64) * t91081;
    let t93899 = F::cast_from(0.52089578783527170489e-1_f64) * t91531;
    (t93595, t93605, t93615, t93650, t93656, t93721, t93723, t93757, t93760, t93795, t93796, t93899)
}
