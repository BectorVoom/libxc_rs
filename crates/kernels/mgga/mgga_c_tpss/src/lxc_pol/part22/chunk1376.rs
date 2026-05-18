//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1376/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1376<F: Float>(t19411: F, t5791: F, t19414: F, t19417: F, t1792: F, t18649: F, t18663: F, t19388: F, t5785: F, t5794: F, t6080: F, t65234: F, t65237: F, t65244: F, t65321: F, t65325: F) -> F {
    let t67429 = F::new(32.0) / F::new(9.0) * t19411 * t5791;
    let t67431 = F::new(32.0) / F::new(9.0) * t19414 * t5791;
    let t67433 = F::new(32.0) / F::new(9.0) * t19417 * t5791;
    let t67434 = -F::new(2.0) / F::new(3.0) * t65234 * t1792 - F::new(4.0) / F::new(3.0) * t65237 * t1792 - F::new(4.0) / F::new(3.0) * t19414 * t5794 - F::new(2.0) / F::new(3.0) * t65244 * t1792 - F::new(4.0) / F::new(3.0) * t19417 * t5794 - F::new(2.0) / F::new(3.0) * t6080 * t18663 - F::new(10.0) / F::new(3.0) * t18649 * t19388 - F::new(10.0) / F::new(3.0) * t5785 * t65321 - F::new(5.0) / F::new(3.0) * t5785 * t65325 + t67429 + t67431 + t67433;
    t67434
}
