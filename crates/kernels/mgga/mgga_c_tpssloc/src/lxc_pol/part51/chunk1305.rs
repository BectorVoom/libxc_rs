//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1305/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1305<F: Float>(t2098: F, t7774: F, t33627: F, t580: F, t1858: F, t8646: F, t116014: F, t116028: F, t116036: F, t120857: F, t122765: F, t122774: F, t122797: F, t122820: F, t122847: F, t122852: F, t1398: F, t2029: F, t2099: F, t2105: F, t26510: F, t26555: F, t27241: F, t3: F, t5381: F, t7020: F, t7223: F, t7946: F, t8647: F) -> (F,) {
    let t122853 = t2098 * t7774;
    let t122856 = t33627 * t580;
    let t122857 = t8646 * t1858;
    let t122858 = t7946 * t7020 + t120857 + t2099 * t26555 + t27241 * t2029 + t26510 * t2105 + t8647 * t5381 + t116014 + t1398 * (t122774 + t122797 + t122820 + t122847) + t7223 * t7774 + t116028 + t116036 + t122852 + t122853 + t3 * t122765 * t580 + t122856 + t122857;
    (t122858,)
}
