//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1287/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1287<F: Float>(t114864: F, t214: F, t7084: F, t31329: F, t6547: F, t23030: F, t31319: F, t23168: F, t31367: F, t114790: F, t23164: F, t6555: F) -> (F, F, F, F, F, F) {
    let t114865 = F::cast_from(0.63969658155208805863e-1_f64) * t114864;
    let t114866 = t214 * t7084;
    let t114882 = t6547 * t31329;
    let t114891 = t23030 * t31319;
    let t114892 = F::cast_from(0.26044789391763585244e-1_f64) * t114891;
    let t114900 = t23168 * t31367;
    let t114916 = t23164 * t114790 * t6555;
    (t114865, t114866, t114882, t114892, t114900, t114916)
}
