//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1276/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1276<F: Float>(t13554: F, t5522: F, t18404: F, t6103: F, t6106: F, t7798: F, t10445: F, t13133: F, t13220: F, t13228: F, t1753: F, t18380: F, t19318: F, t20078: F, t2056: F, t3491: F, t3493: F, t5514: F, t5536: F, t5692: F, t626: F, t63712: F, t63715: F, t63718: F, t63725: F, t63728: F, t63730: F, t63740: F, t63742: F, t645: F) -> (F,) {
    let t63744 = 4.0 * t13554 * t5522;
    let t63746 = 2.0 * t6103 * t18404;
    let t63748 = 2.0 * t7798 * t6106;
    let t63749 = -2.0 * t13220 * t1753 * t626 - 4.0 * t20078 * t626 * t645 - t10445 * t1753 - 4.0 * t13133 * t5536 - 2.0 * t13228 * t5514 - 4.0 * t18380 * t3493 - 4.0 * t19318 * t2056 - 2.0 * t3491 * t5692 + t63712 - t63715 - t63718 - t63725 - t63728 - t63730 - t63740 - t63742 - t63744 - t63746 - t63748;
    (t63749,)
}
