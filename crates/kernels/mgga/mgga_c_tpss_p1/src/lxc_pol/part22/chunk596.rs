//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 596/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk596<F: Float>(t1985: F, t2644: F, t926: F, t359: F, t361: F, t651: F, t355: F, t350: F, t40: F, t586: F, t339: F, t349: F) -> (F, F, F, F, F, F) {
    let t2645 = t2644 * t1985;
    let t2646 = t926 * t2645;
    let t2650 = t359 * t651 * t361;
    let t2652 = t355 * t2650 / F::new(13824.0);
    let t2655 = F::new(1.0) / t40 / t350 / t586;
    let t2657 = t339 * t349 * t2655;
    (t2645, t2646, t2650, t2652, t2655, t2657)
}
