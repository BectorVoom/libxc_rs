//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1337/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1337<F: Float>(t12367: F, t6013: F, t3025: F, t6496: F, t12317: F, t19084: F, t12384: F, t6002: F, t12325: F, t12330: F, t12363: F, t12374: F, t12391: F, t12401: F, t12516: F, t12520: F, t12524: F, t20837: F, t3103: F, t63292: F, t63294: F, t63296: F, t63298: F, t63300: F, t63314: F, t63318: F) -> (F,) {
    let t68464 = t6013 * t12367;
    let t68466 = t6496 * t3025;
    let t68469 = t19084 * t12317 / 1728.0;
    let t68472 = t6002 * t12384;
    let t68474 = t20837 * t3103 / 216.0 + t63292 / 648.0 - t63294 / 864.0 - t63314 * t12391 / 576.0 - t6013 * t12516 / 576.0 - t6013 * t12520 / 1152.0 - t6013 * t12401 / 384.0 - t19084 * t12325 / 2304.0 - t63314 * t12330 / 1152.0 - t6013 * t12363 / 2304.0 + t63296 / 2304.0 - t63298 / 2304.0 - t6002 * t12524 / 288.0 - t63300 / 1728.0 + t68464 / 10368.0 + t63318 + t68466 / 162.0 - t68469 - t19084 * t12374 / 576.0 + t68472 / 1296.0;
    (t68474,)
}
