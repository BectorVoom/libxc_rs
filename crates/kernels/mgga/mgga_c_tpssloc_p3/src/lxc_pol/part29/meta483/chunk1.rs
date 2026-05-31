//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1824/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1824<F: Float>(t2127: F, t3545: F, t3475: F, t460: F, t7320: F, t2132: F, t607: F, t2136: F, t3535: F, t7338: F) -> (F, F, F, F, F) {
    let t24704 = t2127 * t3545 / F::cast_from(432.0_f64);
    let t24705 = t3475 * t460;
    let t24706 = t24705 * t7320;
    let t24711 = t2132 * t607;
    let t24712 = t24711 * t2136;
    let t24716 = t3535 * t7338;
    (t24704, t24705, t24706, t24712, t24716)
}
