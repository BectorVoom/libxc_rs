//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2345/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2345<F: Float>(t1827: F, t80914: F, t1811: F, t80775: F, t7709: F, t80766: F, t22797: F, t5227: F, t22804: F, t26277: F, t80940: F, t16308: F, t22833: F) -> (F, F, F, F, F, F, F) {
    let t91394 = t80914 * t1827;
    let t91398 = t80775 * t1811;
    let t91400 = t80766 * t7709;
    let t91402 = t22797 * t5227;
    let t91403 = F::new(7.0) / F::new(72.0) * t91402;
    let t91404 = t22804 * t26277;
    let t91406 = F::cast_from(0.22608743412718618878e-1_f64) * t80940;
    let t91413 = t22833 * t16308;
    (t91394, t91398, t91400, t91403, t91404, t91406, t91413)
}
