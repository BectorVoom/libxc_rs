//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2119/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2119<F: Float>(t1877: F, t2219: F, t6666: F, t25353: F, t2752: F, t25213: F, t6547: F, t22986: F, t23270: F, t25053: F, t2553: F, t4119: F, t857: F) -> (F, F, F, F, F) {
    let t86835 = F::new(2.0) * t1877 * t6666 * t2219;
    let t86836 = t25353 * t2752;
    let t86843 = t6547 * t25213;
    let t86844 = F::cast_from(0.38381794893125283518e-1_f64) * t86843;
    let t86847 = t22986 * t23270 * t25053 * t2553;
    let t86849 = t857 * t4119;
    (t86835, t86836, t86844, t86847, t86849)
}
