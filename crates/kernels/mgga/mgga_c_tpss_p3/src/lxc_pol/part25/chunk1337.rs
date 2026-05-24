//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1337/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1337<F: Float>(t1791: F, t69198: F, t69206: F, t19349: F, t67472: F, t1792: F, t18350: F, t18666: F, t18673: F, t19342: F, t21136: F, t25752: F, t5794: F, t62294: F, t67378: F, t67385: F, t69087: F, t69195: F, t69203: F, t69210: F, t7690: F) -> F {
    let t71396 = t1791 * t69198;
    let t71401 = t1791 * t69206;
    let t71404 = t19349 * t67472;
    let t71411 = -F::new(40.0) * t7690 * t25752 * t19342 - F::new(2.0) / F::new(3.0) * t69087 * t1792 - F::new(2.0) / F::new(3.0) * t21136 * t5794 + F::new(20.0) * t18666 * t69195 + F::new(20.0) / F::new(3.0) * t18350 * t71396 + F::new(10.0) * t18666 * t69203 + F::new(10.0) / F::new(3.0) * t18350 * t71401 - F::new(160.0) / F::new(9.0) * t71404 + F::new(10.0) / F::new(3.0) * t69210 * t18673 + F::new(20.0) / F::new(3.0) * t19349 * t67378 - t62294 + F::new(176.0) / F::new(27.0) * t67385;
    t71411
}
