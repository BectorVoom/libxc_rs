//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1325/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1325<F: Float>(t1299: F, t2016: F, t10353: F, t10412: F, t10416: F, t11476: F, t1675: F, t1679: F, t18305: F, t19213: F, t19229: F, t19404: F, t19408: F, t1985: F, t1992: F, t20760: F, t20768: F, t20769: F, t20772: F, t20780: F, t5483: F, t5492: F, t5506: F, t5966: F, t5971: F, t6077: F, t61976: F, t63521: F, t63530: F, t63534: F, t63556: F, t6472: F, t65321: F, t72: F) -> (F,) {
    let t68056 = t1299 * t2016;
    let t68091 = -t18305 * t6472 / 6.0 - t5483 * t20769 / 3.0 - t5483 * t20772 / 3.0 - t1675 * (-20.0 / 27.0 * t68056 * t1985 + 20.0 / 9.0 * t20760 * t1992 + 5.0 / 108.0 * t63556 * t11476 + 5.0 / 9.0 * t19213 * t10416 + 5.0 / 18.0 * t19213 * t10412 - 5.0 / 6.0 * t5971 * t10353 + t61976) * t72 * t1679 / 6.0 - t1675 * t20768 * t5506 / 3.0 + 5.0 / 3.0 * t5966 * t65321 + 2.0 / 3.0 * t5492 * t20780 + 5.0 / 6.0 * t63521 * t6077 + 5.0 / 3.0 * t63530 * t6077 + 5.0 / 3.0 * t19229 * t19404 + 5.0 / 3.0 * t19229 * t19408 + 5.0 / 6.0 * t63534 * t6077;
    (t68091,)
}
