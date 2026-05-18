//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 985/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk985<F: Float>(t75736: F, t75739: F, t1550: F, t2228: F, t2347: F, t69130: F, t2211: F, t8794: F, t118: F, t15547: F, t2085: F, t76143: F) -> (F, F, F, F, F, F, F, F) {
    let t77824 = F::new(0.10909864661698136691e0) * t75736;
    let t77825 = F::new(0.21819729323396273382e0) * t75739;
    let t77827 = t1550 * t2228 * t2347;
    let t77828 = F::new(0.2993560425465952141e-1) * t77827;
    let t77830 = F::new(0.18183107769496894487e-1) * t69130;
    let t77831 = t2211 * t8794;
    let t77833 = F::new(0.39914139006212695214e-1) * t118 * t77831;
    let t77834 = t15547 * t2085;
    let t77835 = F::new(0.90915538847484472429e-2) * t77834;
    let t77836 = F::new(0.44903406381989282115e-1) * t76143;
    (t77824, t77825, t77828, t77830, t77831, t77833, t77835, t77836)
}
