//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1055/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1055<F: Float>(t5: F, t25: F, t265: F, t394: F, t24541: F, t112: F, t671: F, t7408: F, t2165: F, t2363: F, t23772: F, t2116: F, t2250: F, t23309: F, t40: F, t607: F, t7274: F, t3630: F, t7301: F, t7300: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t24542 = piecewise3(t8, 0.0, t24541);
    let t24543 = t24542 * t112;
    let t24545 = t7408 * t671;
    let t24552 = t2165 * t2363;
    let t24555 = piecewise3(t395, 0.0, t23772);
    let t24562 = piecewise3(t115, t23309, t24555 * t40 / 2.0 + t7274 * t607 + t2116 * t2250 / 2.0);
    let t24563 = t7301 * t3630;
    let t24564 = t7300 * t24563;
    (t24542, t24543, t24545, t24552, t24555, t24562, t24563, t24564)
}
