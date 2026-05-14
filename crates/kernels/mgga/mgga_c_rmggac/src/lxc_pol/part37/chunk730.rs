//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 730/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk730<F: Float>(t14020: F, t14117: F, t14123: F, t21052: F, t73712: F, t68357: F, t73717: F, t15394: F, t70548: F, t2060: F, t8794: F, t739: F, t14225: F, t3352: F, t8436: F, t1986: F, t305: F, t8441: F) -> (F, F, F, F, F, F, F) {
    let t75134 = t21052 * t14020 * t14123 * t14117 * t73712;
    let t75137 = t68357 * t14117 * t73717;
    let t75139 = t70548 * t15394;
    let t75141 = t2060 * t8794;
    let t75143 = 0.2993560425465952141e-1 * t739 * t75141;
    let t75145 = t14225 * t3352 * t8436;
    let t75148 = t1986 * t305 * t8441;
    (t75134, t75137, t75139, t75141, t75143, t75145, t75148)
}
