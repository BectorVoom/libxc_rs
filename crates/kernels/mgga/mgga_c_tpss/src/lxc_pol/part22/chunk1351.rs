//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1351/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1351<F: Float>(t66379: F, t66494: F, t66546: F, t66601: F, t823: F, t198: F, t5864: F, t20526: F, t64302: F, t1692: F, t17931: F, t18728: F, t18807: F, t19672: F, t19819: F, t19829: F, t19836: F, t20417: F, t20510: F, t2439: F, t30: F, t580: F, t5849: F, t63841: F, t63847: F, t63850: F, t63864: F, t64241: F, t64260: F, t64263: F, t66311: F, t66317: F) -> (F, F, F, F, F) {
    let t66603 = t66379 + t66494 + t66546 + t66601;
    let t66604 = t66603 * t823;
    let t66608 = t198 * t5864;
    let t66615 = F::new(2.0) * t20526 * t64302;
    let t66618 = F::new(6.0) * t20417 * t64260 + F::new(6.0) * t20417 * t64263 + F::new(3.0) * t20417 * t64241 + F::new(6.0) * t66311 * t19672 - t1692 * t18807 * t19836 - F::new(3.0) * t66317 * t17931 - F::new(3.0) * t20417 * t63864 + t1692 * t20510 * t580 - F::new(3.0) * t18728 * t63847 - F::new(3.0) * t18728 * t63850 + t1692 * t66604 * t30 / F::new(2.0) + F::new(2.0) * t66608 * t19819 + F::new(3.0) * t2439 * t5849 * t19829 - t66615 - F::new(3.0) * t18728 * t63841;
    (t66603, t66604, t66608, t66615, t66618)
}
