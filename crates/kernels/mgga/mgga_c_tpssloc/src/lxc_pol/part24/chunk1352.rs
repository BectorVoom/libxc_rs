//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1352/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1352<F: Float>(t23197: F, t6547: F, t23222: F, t23237: F, t6552: F, t23257: F, t6562: F, t794: F, t10111: F, t1911: F, t22975: F, t23191: F, t23215: F, t2597: F, t2713: F, t2718: F, t2742: F, t40890: F, t6662: F, t82219: F, t82221: F, t82228: F, t855: F) -> F {
    let t82230 = t6547 * t23197;
    let t82233 = t6552 * t23237 * t23222;
    let t82236 = t6562 * t794 * t23257;
    let t82246 = F::new(6.0) * t855 * t2718 * t6662 * t2742 - t82219 + F::new(0.49348022005446793095e-1) * t82221 + F::new(6.0) * t2713 * t22975 - F::new(0.14804406601634037928e0) * t82228 - F::new(0.11514538467937585055e0) * t82230 - F::new(0.49348022005446793095e-1) * t82233 - F::new(0.12337005501361698274e-1) * t82236 - F::new(18.0) * t2597 * t23215 - F::new(3.0) * t2713 * t23191 + F::new(24.0) * t855 * t40890 * t1911 * t10111;
    t82246
}
