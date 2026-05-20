//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 882/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk882<F: Float>(t2180: F, t671: F, t1401: F, t3938: F, t3941: F, t577: F, t8143: F, t8153: F, t8161: F, t1774: F, t1453: F, t8129: F) -> (F, F, F, F) {
    let t8166 = t2180 * t671;
    let t8171 = F::new(0.45e1) * t8153 * t577 + F::new(0.135e2) * t8161 * t671 + F::new(0.135e2) * t3938 * t2180 + F::new(27.0) * t3941 * t8166 + F::new(0.135e2) * t1401 * t8143;
    let t8221 = t1774 * t2180;
    let t8223 = t8129 * t1453;
    (t8166, t8171, t8221, t8223)
}
