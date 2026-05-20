//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 934/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk934<F: Float>(t225: F, t26732: F, t26734: F, t27137: F, t27059: F, t27070: F, t27052: F, t2085: F, t5286: F, t1824: F, t7191: F, t112: F, t27240: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92847 = t26732 * t225;
    let t92939 = t26734 * t225;
    let t93313 = t27137 * t225;
    let t93316 = t27059 * t225;
    let t93338 = t27070 * t225;
    let t93341 = t27052 * t225;
    let t93501 = t2085 * t5286;
    let t93505 = t7191 * t1824;
    let t94127 = t27240 * t112;
    (t92847, t92939, t93313, t93316, t93338, t93341, t93501, t93505, t94127)
}
