//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1038/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1038<F: Float>(t4708: F, t8167: F, t10654: F, t14252: F, t14254: F, t14258: F, t14300: F, t14304: F, t14308: F, t14311: F, t14314: F, t14316: F, t761: F, t771: F, t797: F, t8177: F, t8188: F) -> F {
    let t14318 = t8167 * t4708;
    let t14320 = -F::new(7.0) / F::new(2304.0) * t14252 + F::new(7.0) / F::new(4608.0) * t14254 + F::new(5.0) / F::new(768.0) * t797 * t14258 - t771 * t14300 / F::new(3072.0) - t10654 - t761 * t14304 / F::new(48.0) - F::new(35.0) / F::new(216.0) * t8177 - t8188 - F::new(35.0) / F::new(1152.0) * t14308 - t797 * t14311 / F::new(768.0) + F::new(7.0) / F::new(1152.0) * t14314 + F::new(7.0) / F::new(144.0) * t14316 - F::new(7.0) / F::new(48.0) * t14318;
    t14320
}
