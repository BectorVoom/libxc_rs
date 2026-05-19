//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 824/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk824<F: Float>(t1911: F, t857: F, t776: F, t23270: F, t22986: F, t2717: F, t865: F, t1888: F, t794: F, t8331: F, t6562: F, t225: F, t258: F, t6624: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30622 = t857 * t1911;
    let t30623 = t30622 * t776;
    let t30624 = t23270 * t30623;
    let t30626 = F::cast_from(0.3289868133696452873e-1_f64) * t22986 * t30624;
    let t30633 = t2717 * t1911;
    let t30634 = t30633 * t865;
    let t30635 = t23270 * t30634;
    let t30637 = F::cast_from(0.3289868133696452873e-1_f64) * t1888 * t30635;
    let t30638 = t794 * t8331;
    let t30640 = F::cast_from(0.82246703342411321825e-2_f64) * t6562 * t30638;
    let t30642 = t6624 * t225 * t258;
    (t30622, t30623, t30624, t30626, t30633, t30634, t30635, t30637, t30638, t30640, t30642)
}
