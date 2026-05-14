//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1357/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1357<F: Float>(t1688: F, t41905: F, t42719: F, t13133: F, t5531: F, t42336: F, t13220: F, t1338: F, t17916: F, t19457: F, t19462: F, t2105: F, t3537: F, t5514: F, t61897: F, t63756: F, t645: F, t65082: F, t65429: F, t65490: F, t65518: F) -> (F,) {
    let t65993 = 2.0 * t41905 * t1688;
    let t65995 = 4.0 * t42719 * t1688;
    let t65997 = 4.0 * t13133 * t5531;
    let t65999 = 2.0 * t42336 * t1688;
    let t66001 = 2.0 * t13220 * t5514 + 2.0 * t1338 * t61897 + 4.0 * t1338 * t63756 + 2.0 * t1338 * t65518 + 4.0 * t17916 * t3537 + 4.0 * t19457 * t3537 + 2.0 * t19462 * t2105 + 4.0 * t645 * t65490 + 2.0 * t65082 + t65429 + t65993 + t65995 + t65997 + t65999;
    (t66001,)
}
