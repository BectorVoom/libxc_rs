//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1009/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1009<F: Float>(t1729: F, t8878: F, t1742: F, t493: F, t470: F, t1737: F, t1748: F, t2134: F, t32425: F, t32441: F, t32445: F, t32448: F, t34260: F, t34263: F, t488: F, t7326: F, t8028: F, t8031: F, t8875: F) -> (F, F, F, F) {
    let t34266 = t1729 * t8878;
    let t34271 = t493 * t1742;
    let t34272 = t470 * t34271;
    let t34277 = -0.32298204875312312685e-2 * t8028 * t8875 + t32425 - 0.40372756094140390856e-3 * t8031 * t8875 - 0.40372756094140390856e-3 * t2134 * t34260 + 0.40372756094140390856e-3 * t7326 * t34263 + t34266 * t488 / 1536.0 + t32441 * t1737 / 1536.0 - t34272 * t488 / 288.0 + t32445 - t32448 * t1748 / 2304.0;
    (t34266, t34271, t34272, t34277)
}
