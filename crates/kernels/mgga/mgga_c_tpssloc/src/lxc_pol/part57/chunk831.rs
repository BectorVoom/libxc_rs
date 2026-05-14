//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 831/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk831<F: Float>(t33448: F, t81591: F, t6562: F, t8547: F, t86893: F, t214: F, t7823: F, t225: F, t33412: F, t33371: F, t6547: F, t33458: F, t6579: F, t23185: F, t33457: F, t82074: F) -> (F, F, F, F, F, F, F) {
    let t121371 = t81591 * t33448;
    let t121399 = t6562 * t86893 * t8547;
    let t121401 = t214 * t7823;
    let t121405 = t33412 * t225;
    let t121431 = t6547 * t33371;
    let t121437 = t6579 * t33458;
    let t121444 = t23185 * t82074 * t33457;
    (t121371, t121399, t121401, t121405, t121431, t121437, t121444)
}
